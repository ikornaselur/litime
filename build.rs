use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

const TIMES_PATH: &str = "external/literature-clock/docs/times/";

#[derive(Debug, Deserialize)]
struct Quote {
    quote_first: String,
    quote_time_case: String,
    quote_last: String,
    title: String,
    author: String,
    sfw: String,
}

fn get_quotes() -> HashMap<String, Vec<Quote>> {
    let mut map: HashMap<String, Vec<Quote>> = HashMap::new();

    // Walk through the times directory and get all the files, sorted by name
    for file in fs::read_dir(TIMES_PATH).unwrap() {
        let file = file.unwrap();
        let file_name = file.file_name();

        // Split the file name into name and extension
        let file_name_string = file_name.into_string().unwrap();
        let (key, ext) = file_name_string.split_once('.').unwrap();
        let key = key.to_string();

        if ext == "json" {
            let contents = fs::read_to_string(file.path()).unwrap();
            let quotes: Vec<Quote> = serde_json::from_str(&contents).unwrap();
            map.insert(key, quotes);
        }
    }

    map
}

/// Get the next timestamp
///
/// The next timestamp is one minute later, for example:
///
///     get_next_timestamp("00_00") == "00_01"
///     get_next_timestamp("00_59") == "01_00"
///     get_next_timestamp("23_59") == "00_00"
fn get_next_timestamp(ts: String) -> String {
    let mut ts = ts.split('_');
    let mut hours = ts.next().unwrap().parse::<u8>().unwrap();
    let mut minutes = ts.next().unwrap().parse::<u8>().unwrap();
    minutes += 1;
    if minutes == 60 {
        hours += 1;
        minutes = 0;
    }
    if hours == 24 {
        hours = 0;
    }
    format!("{:02}_{:02}", hours, minutes)
}

fn replace(s: &str) -> String {
    s.replace('"', "\\\"")
        .replace("<br>", "\\n")
        .replace("<br/>", "\\n")
        .replace("<br />", "\\n")
        .replace("\\n ", "\\n")
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let quotes_dest_path = Path::new(&out_dir).join("quotes.rs");
    let options_dest_path = Path::new(&out_dir).join("options.rs");

    let quotes = get_quotes();

    // Create the quotes.rs file by iterating through the quotes, and writing
    // the quote data to the file
    //
    // The structure is as follows, for each key (in the form of XX_YY) write a single line for
    // every quote Z that looks like:
    //
    // static QUOTE_XX_YY_Z: Minute = Minute{title: quote.title, author: quote.author, start: quote.quote_first, time: quote.quote_time_case, end: quote.quote_last};
    let mut quotes_string = String::new();
    for (key, value) in quotes.iter() {
        for (index, quote) in value.iter().enumerate() {
            quotes_string.push_str(&format!(
                "static QUOTE_{}_{}: Minute = Minute{{title: \"{}\", author: \"{}\", start: \"{}\", time: \"{}\", end: \"{}\"}};\n",
                key,
                index,
                replace(&quote.title),
                replace(&quote.author),
                replace(&quote.quote_first),
                replace(&quote.quote_time_case),
                replace(&quote.quote_last),
            ));
        }
    }
    fs::write(quotes_dest_path, quotes_string).unwrap();

    // Create the options.rs file by iterating through the quotes, and writing
    // the options data to the file
    //
    // The structure is as follows, for each key (in the form of XX_YY) write two lines,
    // one is for sfw quotes and one for not sfw quotes, that looks like:
    //
    // ("XX:YY", false) => vec![&QUOTE_XX_YY_0, &QUOTE_XX_YY_1, ...],
    // ("XX:YY", true) => vec![&QUOTE_XX_YY_0, &QUOTE_XX_YY_1, ...],
    //
    // Only include the quotes that are sfw if the sfw flag is true, when the flag is false, it can
    // include all the quotes
    let mut pre_processed_matches: Vec<(String, String, String)> = Vec::new();

    // Convert the quotes hashmap to a sorted list of (key, value) tuples
    let mut values = quotes.iter().collect::<Vec<_>>();
    values.sort_by(|a, b| a.0.cmp(b.0));

    for (key, value) in values {
        let mut sfw_quotes: Vec<String> = Vec::new();
        let mut all_quotes: Vec<String> = Vec::new();

        for (index, quote) in value.iter().enumerate() {
            all_quotes.push(format!("&QUOTE_{}_{}", key, index));
            if quote.sfw == "yes" {
                sfw_quotes.push(format!("&QUOTE_{}_{}", key, index));
            }
        }

        // Check if we need to fill future gaps
        let mut timestamps: Vec<String> = vec![key.to_string()];
        let mut timestamp = get_next_timestamp(key.clone());
        while !quotes.contains_key(&timestamp) {
            timestamps.push(timestamp.clone());
            timestamp = get_next_timestamp(timestamp);
        }
        let joined_timestamps = timestamps.join("\" | \"");

        if sfw_quotes.len() == all_quotes.len() {
            // We only have SFW quotes, so just point timestamps at all quotes
            pre_processed_matches.push((joined_timestamps, "_".to_string(), all_quotes.join(", ")));
        } else if sfw_quotes.is_empty() {
            // We have no SFW quotes. Add the quotes for NSFW and then add the
            // timestamp to previous step for SFW Pop off the last value added
            let (mut previous_joined_timestamps, sfw, varnames) =
                pre_processed_matches.pop().unwrap();
            if sfw == "true" {
                // We can just add it
                previous_joined_timestamps += &("\" | \"".to_string() + &joined_timestamps);
                // push it back on
                pre_processed_matches.push((previous_joined_timestamps, sfw, varnames));
            } else if sfw == "_" {
                // We have to split it
                // Push on nsfw to be the normal
                pre_processed_matches.push((
                    previous_joined_timestamps.clone(),
                    "false".to_string(),
                    varnames.clone(),
                ));
                // Push on new sfw to be normal + new
                previous_joined_timestamps += &("\" | \"".to_string() + &joined_timestamps);
                pre_processed_matches.push((
                    previous_joined_timestamps,
                    "true".to_string(),
                    varnames,
                ));
            } else {
                panic!("Last preprocessed was nsfw?");
            }

            // And then deal with nsfw as normal
            pre_processed_matches.push((
                joined_timestamps,
                "false".to_string(),
                all_quotes.join(", "),
            ));
        } else {
            // It's a mix
            // Point at all for nsfw
            pre_processed_matches.push((
                joined_timestamps.clone(),
                "false".to_string(),
                all_quotes.join(", "),
            ));
            // Point at sfw for sfw
            pre_processed_matches.push((
                joined_timestamps,
                "true".to_string(),
                sfw_quotes.join(", "),
            ));
        }
    }
    // Join all into options
    let mut options_string = String::new();
    options_string.push_str("match (time, sfw) {\n");

    for (time, sfw, quotes) in pre_processed_matches {
        options_string.push_str(&format!(
            "(\"{}\", {}) => vec![{}],\n",
            time.replace('_', ":"),
            sfw,
            quotes,
        ));
    }
    options_string.push_str("(_, _) => bail!(\"Couldn't match timestamp!\"),\n");
    options_string.push('}');

    fs::write(options_dest_path, options_string).unwrap();

    println!("cargo:rerun-if-changed=external/literature-clock/");
    println!("cargo:rerun-if-changed=build.rs");
}

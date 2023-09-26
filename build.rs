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
        let key = key.clone().to_string();

        if ext == "json" {
            let contents = fs::read_to_string(file.path()).unwrap();
            let quotes: Vec<Quote> = serde_json::from_str(&contents).unwrap();
            map.insert(key, quotes);
        }
    }

    map
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
                quote.title.replace('"', "\\\""),
                quote.author.replace('"', "\\\""),
                quote.quote_first.replace('"', "\\\""),
                quote.quote_time_case.replace('"', "\\\""),
                quote.quote_last.replace('"', "\\\"")
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
    let mut options_string = String::new();
    options_string.push_str("match (time, sfw) {\n");

    println!("cargo:warning=Need to fill gaps, see the 'check if we need to fill future gaps' in script.py");

    for (key, value) in quotes.iter() {
        let mut sfw_quotes: Vec<String> = Vec::new();
        let mut all_quotes: Vec<String> = Vec::new();

        for (index, quote) in value.iter().enumerate() {
            all_quotes.push(format!("&QUOTE_{}_{}", key, index));
            if quote.sfw == "yes" {
                sfw_quotes.push(format!("&QUOTE_{}_{}", key, index));
            }
        }

        if sfw_quotes.is_empty() {
            println!(
                "cargo:warning=No SFW quotes for {}, need to point at previous timestamp",
                key
            );
        }
        if sfw_quotes.len() == all_quotes.len() {
            options_string.push_str(&format!(
                "(\"{}\", _) => vec![{}],\n",
                key.replace('_', ":"),
                all_quotes.join(", ")
            ));
        } else {
            options_string.push_str(&format!(
                "(\"{}\", false) => vec![{}],\n",
                key.replace('_', ":"),
                all_quotes.join(", ")
            ));
            options_string.push_str(&format!(
                "(\"{}\", true) => vec![{}],\n",
                key.replace('_', ":"),
                sfw_quotes.join(", ")
            ));
        }
    }
    options_string.push_str("(_, _) => bail!(\"Couldn't match timestamp!\"),\n");
    options_string.push('}');

    fs::write(options_dest_path, options_string).unwrap();

    println!("cargo:rerun-if-changed=external/literature-clock/");
    println!("cargo:rerun-if-changed=build.rs");
}

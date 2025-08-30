use anyhow::{bail, Result};
use clap::Parser;
use termcolor::Color;
use terminal_size::{terminal_size, Width};
use time::{format_description, OffsetDateTime, Time, UtcOffset};
use time_tz::{timezones::get_by_name, OffsetDateTimeExt, Tz};

use litime::formatter::{Formatting, Style, FORMATTING_HELP};
use litime::minute::get_minute;

static MIN_WIDTH: u16 = 20;
static MAX_WIDTH: u16 = 120;
static DEFAULT_WIDTH: u16 = 80; // If we fail to get terminal width
static MARGIN: u16 = 5; // Margin on the right of the comment, only used when automatically detecting the terminal width

/// Hello
#[derive(Parser, Debug)]
#[command(
    version,
    about = r#"A command line tool to dispaly the current time-ish with a literature quote.

Parts of the quote can be formatted with formatting strings, in the form of '<style> <colour>' or just '<colour>'
Available colours are:

    black, white, blue, cyan, green, magenta, red and yellow

Available styles are:

    bold, dimmed, intense, italic, strikethrough and underline 
"#
)]
struct Args {
    /// A timestamp to get a quote for
    ///
    /// The timestamp should be a 24-hour timestamp in the form of HH:MM, such as 07:13 or 21:20.
    /// If the timestamp is not given, the current time of the system is used.
    #[arg(value_parser = is_timestamp)]
    time: Option<String>,

    /// Formatting of the quote, excluding the time
    #[arg(long, short, value_parser = is_valid_format, default_value_t = Formatting {
        style: Style::Plain,
        colour: Color::White
    })]
    main_formatting: Formatting,

    /// Formatting of the time part of the quote
    #[arg(long, short, value_parser = is_valid_format, default_value_t = Formatting {
        style: Style::Intense,
        colour: Color::Red
    })]
    time_formatting: Formatting,

    /// Formatting of the author and book below the quote
    #[arg(long, short, value_parser = is_valid_format, default_value_t = Formatting {
        style: Style::Intense,
        colour: Color::White
    })]
    author_formatting: Formatting,

    /// Whether only to show SFW quotes
    #[arg(long, short)]
    sfw: bool,

    /// The number of columns for the quote, before wrapping into another line.
    ///
    /// If not set, then litime will try to detect the width of the terminal window, up to `MAX_WIDTH` (default 120 columns)
    #[arg(short, long)]
    width: Option<u16>,

    /// The max width of the quote when not set by the WIDTH option
    #[arg(long, default_value_t = MAX_WIDTH)]
    max_width: u16,

    /// Override the time zone used when no explicit time is provided
    ///
    /// Accepted values: 'local', 'utc', an IANA region like 'Europe/Stockholm',
    /// or a numeric UTC offset like '+02:00', '-0730', '+00'.
    /// See https://time.now/iana-time-zone/ for list of options
    #[arg(long, value_parser = parse_tz)]
    tz: Option<TzOverride>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let timestamp = args.time.unwrap_or_else(|| {
        let now = match &args.tz {
            None | Some(TzOverride::Local) => OffsetDateTime::now_local().unwrap(),
            Some(TzOverride::Utc) => OffsetDateTime::now_utc(),
            Some(TzOverride::Offset(offset)) => OffsetDateTime::now_utc().to_offset(*offset),
            Some(TzOverride::Region(tz)) => OffsetDateTime::now_utc().to_timezone(*tz),
        };
        format!("{:0width$}:{:0width$}", now.hour(), now.minute(), width = 2)
    });
    let minute = get_minute(&timestamp, args.sfw)?;
    let max_width = args.max_width;

    let width = args.width.unwrap_or_else(|| match terminal_size() {
        Some((Width(width), _)) => std::cmp::min(width, max_width) - MARGIN,
        None => DEFAULT_WIDTH,
    });

    println!(
        "{}",
        minute.formatted(
            std::cmp::max(width, MIN_WIDTH),
            &args.main_formatting,
            &args.time_formatting,
            &args.author_formatting,
        )?
    );

    Ok(())
}

fn is_timestamp(val: &str) -> Result<String> {
    let format = format_description::parse("[hour]:[minute]")?;
    match Time::parse(val, &format) {
        Ok(_) => Ok(val.to_string()),
        _ => bail!("The value must be a valid 24-hour timestamp in the format HH:MM"),
    }
}

#[derive(Clone, Debug)]
enum TzOverride {
    Local,
    Utc,
    Offset(UtcOffset),
    Region(&'static Tz),
}

fn parse_tz(val: &str) -> Result<TzOverride> {
    let lower = val.to_ascii_lowercase();
    if lower == "local" {
        return Ok(TzOverride::Local);
    }
    if lower == "utc" || lower == "z" {
        return Ok(TzOverride::Utc);
    }

    // Try IANA region (case-sensitive names usually include '/'; we try original input)
    if let Some(tz) = get_by_name(val) {
        return Ok(TzOverride::Region(tz));
    }

    // Expect formats like +HH, +HHMM, +HH:MM (and negative variants)
    let bytes = lower.as_bytes();
    if bytes.is_empty() || (bytes[0] != b'+' && bytes[0] != b'-') {
        bail!("Invalid time zone. Use 'local', 'utc', IANA region (e.g., 'Europe/Stockholm'), or an offset like '+02:00'.");
    }
    let sign = if bytes[0] == b'-' { -1 } else { 1 };
    let rest = &lower[1..];
    let compact = rest.replace(':', "");
    if compact.len() != 2 && compact.len() != 4 {
        bail!("Invalid UTC offset. Use '+HH', '+HHMM' or '+HH:MM'.");
    }
    let hours: i8 = compact[0..2]
        .parse::<i8>()
        .map_err(|_| anyhow::anyhow!("Invalid hour in UTC offset"))?;
    let minutes: i8 = if compact.len() == 4 {
        compact[2..4]
            .parse::<i8>()
            .map_err(|_| anyhow::anyhow!("Invalid minutes in UTC offset"))?
    } else {
        0
    };
    if !(0..=23).contains(&hours) || !(0..=59).contains(&minutes) {
        bail!("UTC offset out of range. Hours 0-23, minutes 0-59.");
    }
    let offset = if sign < 0 {
        UtcOffset::from_hms(-hours, minutes, 0)?
    } else {
        UtcOffset::from_hms(hours, minutes, 0)?
    };
    Ok(TzOverride::Offset(offset))
}

fn color_from_str(value: &str) -> Result<Color> {
    match value {
        "black" => Ok(Color::Black),
        "blue" => Ok(Color::Blue),
        "cyan" => Ok(Color::Cyan),
        "green" => Ok(Color::Green),
        "magenta" => Ok(Color::Magenta),
        "red" => Ok(Color::Red),
        "white" => Ok(Color::White),
        "yellow" => Ok(Color::Yellow),
        _ => bail!("Invalid colour"),
    }
}

/// Check if value is a valid format
///
/// A valid format is either a colour or a format and a colour separated by a space.
///
/// # Examples
/// ```
/// assert!(is_format("red").is_ok());
/// assert!(is_format("bold red").is_ok());
/// assert!(is_format("bold").is_err());
/// assert!(is_format("red bold").is_err());
/// ```
fn is_valid_format(val: &str) -> Result<Formatting> {
    let parts: Vec<&str> = val.split(' ').collect();

    if parts.len() == 1 {
        Ok(color_from_str(parts[0])?.into())
    } else if parts.len() == 2 {
        Ok(Formatting {
            style: Style::try_from(parts[0])?,
            colour: color_from_str(parts[1])?,
        })
    } else {
        bail!("Invalid format.\n\n{}", FORMATTING_HELP)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tz_parse_local_and_utc() {
        assert!(matches!(parse_tz("local").unwrap(), TzOverride::Local));
        assert!(matches!(parse_tz("utc").unwrap(), TzOverride::Utc));
    }

    #[test]
    fn tz_parse_offsets() {
        if let TzOverride::Offset(off) = parse_tz("+02:00").unwrap() {
            let (h, m, s) = off.as_hms();
            assert_eq!(h, 2);
            assert_eq!(m, 0);
            assert_eq!(s, 0);
        } else {
            panic!("Expected offset");
        }

        if let TzOverride::Offset(off) = parse_tz("-0730").unwrap() {
            let (h, m, s) = off.as_hms();
            assert_eq!(h, -7);
            assert_eq!(m, -30);
            assert_eq!(s, 0);
        } else {
            panic!("Expected offset");
        }
    }

    #[test]
    fn tz_parse_region() {
        match parse_tz("Europe/Stockholm").unwrap() {
            TzOverride::Region(_) => {}
            _ => panic!("Expected region tz"),
        }
        match parse_tz("America/New_York").unwrap() {
            TzOverride::Region(_) => {}
            _ => panic!("Expected region tz"),
        }
    }
}

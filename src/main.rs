extern crate clap;
#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use clap::Parser;
use regex::Regex;

use crate::minute::get_minute;

mod formatter;
mod minute;

static COLOUR_HELP: & str = "Available colours are black, red, green, yellow, blue, magenta, cyan, white.\nEach colour can be prefixed with 'bright-'.";
static DEFAULT_MAIN: &str = "bright-black";
static DEFAULT_TIME: &str = "red";
static DEFAULT_AUTHOR: &str = "white";

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// A timestamp to get a quote for, for example 07:16
    #[clap(validator = is_timestamp)]
    time: Option<String>,

    /// Colour of the quote, excluding the time
    #[clap(short, long, validator = is_colour, default_value_t = DEFAULT_MAIN.to_string())]
    main_colour: String,

    /// Colour of the time part of the quote
    #[clap(short, long, validator = is_colour, default_value_t = DEFAULT_TIME.to_string())]
    time_colour: String,

    /// Colour of the author and book below the quote
    #[clap(short, long, validator = is_colour, default_value_t = DEFAULT_AUTHOR.to_string())]
    author_colour: String,

    /// The max width of the quote
    #[clap(short, long, default_value_t = 80)]
    width: usize,
}

fn main() {
    let args = Args::parse();

    let local: DateTime<Local> = Local::now();
    let now = format!(
        "{:0width$}:{:0width$}",
        local.hour(),
        local.minute(),
        width = 2
    );

    let timestamp = args.time.unwrap_or(now);
    let minute = get_minute(&timestamp);

    print!(
        "{}",
        minute.formatted(
            args.width,
            &args.main_colour,
            &args.time_colour,
            &args.author_colour
        )
    );
}

fn is_timestamp(val: &str) -> Result<(), String> {
    let re = Regex::new(r"^([01][0-9]|2[0-3]):[0-5][0-9]$").unwrap();
    if re.is_match(val) {
        Ok(())
    } else {
        Err(String::from(
            "The value must be a valid 24-hour timestamp in the format HH:MM",
        ))
    }
}

fn is_colour(val: &str) -> Result<(), String> {
    let re = Regex::new(r"^(bright-)?(black|red|green|yellow|blue|magenta|cyan|white)$").unwrap();
    if re.is_match(val) {
        Ok(())
    } else {
        Err(format!("Unknown colour.\n{}", COLOUR_HELP))
    }
}

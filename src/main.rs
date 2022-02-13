extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate termsize;

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
static MIN_WIDTH: usize = 20;
static MAX_WIDTH: usize = 120;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// A timestamp to get a quote for, for example 07:16
    #[clap(validator = is_timestamp)]
    time: Option<String>,

    /// Colour of the quote, excluding the time
    #[clap(long, validator = is_colour, default_value_t = DEFAULT_MAIN.to_string())]
    main_colour: String,

    /// Colour of the time part of the quote
    #[clap(long, validator = is_colour, default_value_t = DEFAULT_TIME.to_string())]
    time_colour: String,

    /// Colour of the author and book below the quote
    #[clap(long, validator = is_colour, default_value_t = DEFAULT_AUTHOR.to_string())]
    author_colour: String,

    /// The number of columns for the quote, before wrapping into another line. If not set, then
    /// litime will try to detect the width of the terminal window, up to MAX_WIDTH (default 120
    /// columns)
    #[clap(short, long)]
    width: Option<usize>,

    /// The max width of the quote when now set by the WIDTH option
    #[clap(long, default_value_t = MAX_WIDTH)]
    max_width: usize,
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
    let max_width = args.max_width as u16;

    let width: usize = args.width.unwrap_or(
        termsize::get()
            .map(|size| std::cmp::min(size.cols, max_width) - 10)
            .unwrap() as usize,
    );

    println!("{}", width);

    print!(
        "{}",
        minute.formatted(
            std::cmp::max(width, MIN_WIDTH),
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

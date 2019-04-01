#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use clap::{App, AppSettings, Arg};
use regex::Regex;

use crate::minute::get_minute;

mod formatter;
mod minute;

static COLOUR_HELP: &'static str = "Available colours are black, red, green, yellow, blue, magenta, cyan, white.\nEach colour can be prefixed with 'bright-'.";
static DEFAULT_MAIN: &'static str = "bright-black";
static DEFAULT_TIME: &'static str = "red";
static DEFAULT_AUTHOR: &'static str = "white";

fn main() {
    let matches = App::new("Litime")
        .version(crate_version!())
        .setting(AppSettings::ColoredHelp)
        .about("Display a timestamp with a literature quote. By default, the current time stamp is used.")
        .arg(Arg::with_name("time")
            .short("t")
            .long("time")
            .value_name("time")
            .help("A timestamp to get a quote for, for example 07:16.")
            .validator(is_timestamp)
            .takes_value(true),
        )
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .value_name("width")
            .help("The max width of the quote")
            .default_value("80")
            .takes_value(true),
        )
        .arg(Arg::with_name("main_colour")
             .short("M")
             .long("main-colour")
             .value_name("main_colour")
             .help("Colour of the quote, excluding the time.")
             .default_value(DEFAULT_MAIN)
             .validator(is_colour)
             .takes_value(true),
         )
        .arg(Arg::with_name("time_colour")
             .short("T")
             .long("time-colour")
             .value_name("time_colour")
             .help("Colour of the time part of the quote.")
             .default_value(DEFAULT_TIME)
             .validator(is_colour)
             .takes_value(true),
         )
        .arg(Arg::with_name("author_colour")
             .short("A")
             .long("author-colour")
             .value_name("author_colour")
             .help("Colour of the author and book below the quote.")
             .default_value(DEFAULT_AUTHOR)
             .validator(is_colour)
             .takes_value(true),
         )
        .get_matches();

    let local: DateTime<Local> = Local::now();
    let now = format!(
        "{:0width$}:{:0width$}",
        local.hour(),
        local.minute(),
        width = 2
    );

    let timestamp = matches.value_of("time").unwrap_or(&now);
    let width = value_t!(matches, "width", usize).unwrap_or_else(|e| e.exit());
    let main_colour = matches.value_of("main_colour").unwrap_or(DEFAULT_MAIN);
    let time_colour = matches.value_of("time_colour").unwrap_or(DEFAULT_TIME);
    let author_colour = matches.value_of("author_colour").unwrap_or(DEFAULT_AUTHOR);

    let minute = get_minute(timestamp);
    print!(
        "{}",
        minute.formatted(width, main_colour, time_colour, author_colour)
    );
}

fn is_timestamp(val: String) -> Result<(), String> {
    let re = Regex::new(r"^([01][0-9]|2[0-3]):[0-5][0-9]$").unwrap();
    if re.is_match(&val) {
        Ok(())
    } else {
        Err(String::from(
            "The value must be a valid 24-hour timestamp in the format HH:MM",
        ))
    }
}

fn is_colour(val: String) -> Result<(), String> {
    let re = Regex::new(r"^(bright-)?(black|red|green|yellow|blue|magenta|cyan|white)$").unwrap();
    if re.is_match(&val) {
        Ok(())
    } else {
        Err(format!("Unknown colour.\n{}", COLOUR_HELP).to_string())
    }
}

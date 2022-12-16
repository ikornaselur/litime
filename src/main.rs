use anyhow::{bail, Result};
use clap::Parser;
use time::{format_description, OffsetDateTime, Time};

use litime::minute::get_minute;

static COLOURS: &[&str; 16] = &[
    "bright-black",
    "bright-blue",
    "bright-cyan",
    "bright-green",
    "bright-magenta",
    "bright-red",
    "bright-white",
    "bright-yellow",
    "black",
    "blue",
    "cyan",
    "green",
    "magenta",
    "red",
    "white",
    "yellow",
];

static COLOUR_HELP: &str = "Available colours are black, blue, cyan, green, magenta, red, white and yellow.\nEach colour can be prefixed with 'bright-'.";
static DEFAULT_MAIN: &str = "bright-black";
static DEFAULT_TIME: &str = "red";
static DEFAULT_AUTHOR: &str = "white";
static MIN_WIDTH: usize = 20;
static MAX_WIDTH: usize = 120;
static DEFAULT_WIDTH: usize = 80; // If we fail to get terminal width
static MARGIN: usize = 5; // Margin on the right of the comment, only used when automatically detecting the terminal width

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// A timestamp to get a quote for
    ///
    /// The timestamp should be a 24-hour timestamp in the form of HH:MM, such as 07:13 or 21:20.
    /// If the timestamp is not given, the current time of the system is used.
    #[arg(value_parser = is_timestamp)]
    time: Option<String>,

    /// Colour of the quote, excluding the time
    #[arg(long, short, value_parser = is_colour, default_value_t = DEFAULT_MAIN.to_string())]
    main_colour: String,

    /// Colour of the time part of the quote
    #[arg(long, short, value_parser = is_colour, default_value_t = DEFAULT_TIME.to_string())]
    time_colour: String,

    /// Colour of the author and book below the quote
    #[arg(long, short, value_parser = is_colour, default_value_t = DEFAULT_AUTHOR.to_string())]
    author_colour: String,

    /// Whether only to show SFW quotes
    #[arg(long, short)]
    sfw: bool,

    /// The number of columns for the quote, before wrapping into another line.
    ///
    /// If not set, then litime will try to detect the width of the terminal window, up to MAX_WIDTH (default 120 columns)
    #[arg(short, long)]
    width: Option<usize>,

    /// The max width of the quote when not set by the WIDTH option
    #[arg(long, default_value_t = MAX_WIDTH)]
    max_width: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let timestamp = args.time.unwrap_or_else(|| {
        let now_local = OffsetDateTime::now_local().unwrap();
        format!(
            "{:0width$}:{:0width$}",
            now_local.hour(),
            now_local.minute(),
            width = 2
        )
    });
    let minute = get_minute(&timestamp, args.sfw)?;
    let max_width = args.max_width;

    let width: usize = args.width.unwrap_or_else(|| {
        termsize::get()
            .map(|size| std::cmp::min(size.cols as usize, max_width) - MARGIN)
            .unwrap_or(DEFAULT_WIDTH)
    });

    print!(
        "{}",
        minute.formatted(
            std::cmp::max(width, MIN_WIDTH),
            &args.main_colour,
            &args.time_colour,
            &args.author_colour
        )
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

fn is_colour(val: &str) -> Result<String> {
    if COLOURS.iter().any(|&c| c == val) {
        Ok(val.to_string())
    } else {
        bail!("Unknown colour.\n{}", COLOUR_HELP)
    }
}

use anyhow::{bail, Result};
use clap::Parser;
use time::{format_description, OffsetDateTime, Time};

use litime::formatter::{Colour, Formatting, Style, FORMATTING_HELP};
use litime::minute::get_minute;

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

    /// Formatting of the quote, excluding the time
    #[arg(long, short, value_parser = is_valid_format, default_value_t = Formatting {
        style: Style::Plain,
        colour: Colour::White
    })]
    main_formatting: Formatting,

    /// Formatting of the time part of the quote
    #[arg(long, short, value_parser = is_valid_format, default_value_t = Formatting {
        style: Style::Bold,
        colour: Colour::Red
    })]
    time_formatting: Formatting,

    /// Formatting of the author and book below the quote
    #[arg(long, short, value_parser = is_valid_format, default_value_t = Formatting {
        style: Style::Plain,
        colour: Colour::White
    })]
    author_formatting: Formatting,

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
            &args.main_formatting,
            &args.time_formatting,
            &args.author_formatting
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
        Ok(Colour::try_from(parts[0])?.into())
    } else if parts.len() == 2 {
        Ok(Formatting {
            style: Style::try_from(parts[0])?,
            colour: Colour::try_from(parts[1])?,
        })
    } else {
        bail!("Invalid format.\n\n{}", FORMATTING_HELP)
    }
}

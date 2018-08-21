extern crate chrono;
extern crate colored;
extern crate rand;
extern crate textwrap;

use chrono::prelude::*;
use colored::*;
use textwrap::fill;

use minute::get_minute;

mod minute;

fn main() {
    let local: DateTime<Local> = Local::now();
    let now = format!("{}:{}", local.hour(), local.minute());
    let minute = get_minute(now);
    let result = format!(
        "{}{}{}",
        minute.start.bright_black(),
        minute.time.red(),
        minute.end.bright_black()
    );

    let result = fill(&result, 80);

    let mut lines = result.lines();

    println!("");
    // Print first line with a quote mark
    if let Some(line) = lines.next() {
        println!("  \" {}", line);
    }
    // Then rest of the lines just indented
    while let Some(line) = lines.next() {
        println!("    {}", line);
    }
    println!("");
    println!("        {} - {}", minute.author, minute.title);
}

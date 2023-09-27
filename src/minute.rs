use anyhow::{bail, Context, Result};
use rand::seq::SliceRandom;

pub struct Minute<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub start: &'a str,
    pub time: &'a str,
    pub end: &'a str,
}

include!(concat!(env!("OUT_DIR"), "/quotes.rs"));

pub fn get_minute<'a>(time: &str, sfw: bool) -> Result<&'a Minute> {
    let options = include!(concat!(env!("OUT_DIR"), "/options.rs"));

    let quote = options
        .choose(&mut rand::thread_rng())
        .context("Unable to choose a random quote")?;

    Ok(quote)
}

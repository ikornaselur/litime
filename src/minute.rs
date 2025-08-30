use anyhow::{bail, Result};

pub struct Minute<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub start: &'a str,
    pub time: &'a str,
    pub end: &'a str,
}

include!(concat!(env!("OUT_DIR"), "/quotes.rs"));

pub fn get_minute<'a>(time: &str, sfw: bool) -> Result<&'a Minute<'_>> {
    let options = include!(concat!(env!("OUT_DIR"), "/options.rs"));

    match options.len() {
        1 => Ok(options[0]),
        l => {
            // Use the current millisecond fraction as a pseudo random picker
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?;
            let millis = now.subsec_millis() as usize;

            Ok(options[millis % l])
        }
    }
}

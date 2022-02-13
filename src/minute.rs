use chrono::{Duration, NaiveTime};
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug)]
pub struct Minute {
    pub title: String,
    pub author: String,
    pub start: String,
    pub time: String,
    pub end: String,
}

const RAW_TIMES: &str = include_str!("times.json");

pub fn get_minute(time: &str) -> Minute {
    let minutes: HashMap<&str, Vec<Minute>> = serde_json::from_str(RAW_TIMES).unwrap();

    // Try earlier minutes if requested minute doesn't exist
    let mut naive_time = NaiveTime::parse_from_str(time, "%H:%M").unwrap();
    let mut duration = 0;

    while !minutes.contains_key(&naive_time.format("%H:%M").to_string() as &str) {
        duration += 1;
        naive_time -= Duration::minutes(duration);
    }

    let key = naive_time.format("%H:%M").to_string();

    let minute = minutes
        .get(&key as &str)
        .unwrap_or_else(|| panic!("Couldn't find {:?}", time));

    minute
        .choose(&mut rand::thread_rng())
        .expect("Unable to choose a random quote")
        .clone()
}

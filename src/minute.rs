use rand::{thread_rng, Rng};
use serde_json;
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
    let minutes: HashMap<&str, Vec<Minute>> = serde_json::from_str(&RAW_TIMES).unwrap();

    let minute = minutes
        .get(time)
        .unwrap_or_else(|| panic!("Couldn't find {:?}", time));

    thread_rng()
        .choose(&minute)
        .expect("Unable to choose a random quote")
        .clone()
}

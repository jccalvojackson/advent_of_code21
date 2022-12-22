use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct SectionRecord {
    pub finterval: String,
    pub sinterval: String,
}

pub fn interval_to_set(string_int: String) -> HashSet<i32> {
    let mut split = string_int.split("-");
    let start: i32 = split.next().expect("empty").parse::<i32>().unwrap();
    let end: i32 = split.next().expect("empty").parse::<i32>().unwrap();
    (start..=end).collect::<HashSet<i32>>()
}

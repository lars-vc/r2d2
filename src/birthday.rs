extern crate colored;

// use chrono::prelude::*;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::string::String;

use utils::get_files_path;

struct Person {
    name: String,
    date: String,
}
struct BirthdayJson {
    birthdays: Vec<Person>,
}

pub fn add_bd(name: &String, date_str: &String) {
    // let date = NaiveDate::parse_from_str(date_str.as_str(), "%d/%m/%Y").unwrap();
    // println!("{:?}", date);
    let path: String = get_files_path("birthdays.json");
    // let json
    // let res: BirthdayJson = serde_json::from_str(json).unwrap();
}
pub fn remove_bd(name: &String) {}
pub fn list_bd() {}

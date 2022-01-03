extern crate colored;

// use chrono::prelude::*;
use chrono::{Datelike, Local, NaiveDate};
use colored::*;
use serde::{Deserialize, Serialize};
use std::string::String;

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
// use utils::get_files_path;
#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    date: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct BirthdayJson {
    birthdays: Vec<Person>,
}

pub fn add_bd(name: &String, date_str: &String) {
    let date = NaiveDate::parse_from_str(date_str.as_str(), "%d/%m/%Y").unwrap();
    // println!("{:?}", date);
    let path = get_files_path("birthdays.json")
        .expect("Couldn't resolve file path")
        .into_os_string();
    let json: String = fs::read_to_string(&path).expect("Couldn't read json file");
    let mut res: BirthdayJson = serde_json::from_str(json.as_str()).unwrap();
    let p = Person {
        name: name.clone(),
        date: date.format("%d/%m/%Y").to_string(),
    };
    res.birthdays.push(p);
    fs::write(path, serde_json::to_string(&res).unwrap()).expect("Couldn't write out json");
}

pub fn remove_bd(name: &String) {
    let path = get_files_path("birthdays.json")
        .expect("Couldn't resolve file path")
        .into_os_string();
    let json: String = fs::read_to_string(&path).expect("Couldn't read json file");
    let mut res: BirthdayJson = serde_json::from_str(json.as_str()).unwrap();
    res.birthdays.retain(|x| x.name != name.to_string());
    fs::write(path, serde_json::to_string(&res).unwrap()).expect("Couldn't write out json");
}

pub fn list_bd() {
    let path = get_files_path("birthdays.json")
        .expect("Couldn't resolve file path")
        .into_os_string();
    let json: String = fs::read_to_string(&path).expect("Couldn't read json file");
    let mut res: BirthdayJson = serde_json::from_str(json.as_str()).unwrap();
    res.birthdays.sort_by(|x, y| x.name.cmp(&y.name));
    for el in res.birthdays {
        println!("{}, {}", el.name, el.date);
    }
}

pub fn today_bd() {
    let path = get_files_path("birthdays.json")
        .expect("Couldn't resolve file path")
        .into_os_string();
    let json: String = fs::read_to_string(&path).expect("Couldn't read json file");
    let res: BirthdayJson = serde_json::from_str(json.as_str()).unwrap();
    let date = Local::now();
    for el in res.birthdays {
        let shortdate = &el.date[0..6];
        if shortdate == date.format("%d/%m/").to_string() {
            println!(
                "Today is the birthday of {}, they are turning {} ({})🎂",
                el.name.blue(),
                date.year() - &el.date[6..].parse().unwrap(),
                el.date
            );
        }
    }
}

pub fn tmrw_bd() {
    let path = get_files_path("birthdays.json")
        .expect("Couldn't resolve file path")
        .into_os_string();
    let json: String = fs::read_to_string(&path).expect("Couldn't read json file");
    let res: BirthdayJson = serde_json::from_str(json.as_str()).unwrap();
    let date = Local::now().date().succ();
    for el in res.birthdays {
        let shortdate = &el.date[0..6];
        if shortdate == date.format("%d/%m/").to_string() {
            println!(
                "Tomorrow is the birthday of {}, they are turning {} ({})",
                el.name.blue(),
                date.year() - &el.date[6..].parse().unwrap(),
                el.date
            );
        }
    }
}

pub fn query_bd(name: &String) {
    let path = get_files_path("birthdays.json")
        .expect("Couldn't resolve file path")
        .into_os_string();
    let json: String = fs::read_to_string(&path).expect("Couldn't read json file");
    let mut res: BirthdayJson = serde_json::from_str(json.as_str()).unwrap();
    res.birthdays.sort_by(|x, y| x.name.cmp(&y.name));
    for el in res.birthdays {
        if el.name.to_lowercase().contains(&name.to_lowercase()) {
            println!("{}, {}", el.name, el.date);
        }
    }
}

fn get_files_path(endpath: &str) -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("files");
    dir.push(endpath);
    Ok(dir)
}

extern crate getopts;
mod initproj;
use getopts::Options;
use std::env;

fn print_help() {
    println!("The assistant R2D2!");
    println!("ART");
    println!("Usages:");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" | "i" => parse_init(args[2..].to_vec()),
        _ => print_help(),
    }
}

fn parse_init(args: Vec<String>) {
    fn init_help() {
        println!("Insert help menu with langs and options here");
    }
    if args.len() == 0 {
        println!("No arguments giving to init");
        init_help();
        return;
    }
    let lang: std::string::String = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("g", "git", "git url", "URL");
    opts.optflag("n", "nvim", "init project with nvim editor in mind");
    opts.optflag("h", "help", "print help menu of init");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            init_help();
            return;
        }
    };
    if matches.opt_present("h") {
        init_help();
        return;
    }

    let optgit = matches.opt_str("g");
    let giturl: std::string::String = match optgit {
        Some(x) => x,
        None => std::string::String::from(""),
    };
    let nvim: bool = matches.opt_present("n");
    initproj::init_project(lang, nvim, giturl);
    return;
}

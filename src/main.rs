extern crate colored;
extern crate getopts;

mod birthday;
mod initproj;
mod utils;

use colored::*;
use getopts::Options;
use std::env;

fn print_help() {
    println!("The assistant R2D2!");
    print_ascii_art();
    println!("Usages:");
    println!("    init     | init a project for given language (see r2d2 init -h for more info)");
    println!("    task     | Not yet implemented");
    println!("    birthday | save the birthdays of people");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{} {}", "error:".bold().red(), "No arguments given to R2D2");
        print_help();
        return;
    }
    match args[1].as_str() {
        "init" | "i" => parse_init(args[2..].to_vec()),
        "birthday" | "bd" => parse_birthday(args[2..].to_vec()),
        _ => print_help(),
    }
}

fn parse_init(args: Vec<String>) {
    // Error handling
    fn init_help() {
        println!("R2D2 init help");
        println!("How to run:");
        println!("   r2d2 init LANG --OPTIONS | init a project for given language (LANG) with options (OPTIONS)");
        println!("\nOptions:");
        println!("   -n, --neovim      | will create a .vimspector.json file");
        println!("   -g URL, --git URL | will setup remote and push init files to git");
        println!("\nSupported languages:");
        println!("   python, py");
        println!("   rust");
        println!("   c");
        println!("   c++, cpp");
    }
    if args.len() == 0 {
        println!("{} {}", "error:".red().bold(), "No arguments given to init");
        init_help();
        return;
    }
    let lang: std::string::String = args[0].clone();

    // Not very clean but Ill keep it for now
    if lang.chars().nth(0).unwrap() == '-' && lang.chars().nth(1).unwrap() == 'h' {
        init_help();
        return;
    }
    // doesnt work v
    // if std::string::String::from(&lang[0..1]) == std::string::String::from("-h") {
    //     init_help();
    //     return;
    // }

    // GETOPTS
    let mut opts = Options::new();
    opts.optopt("g", "git", "git url", "URL");
    opts.optflag("n", "nvim", "init project with nvim editor in mind");
    opts.optflag("h", "help", "print help menu of init");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{} {}", "error:".bold().red(), f.to_string());
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

    // send it off
    initproj::init_project(lang, nvim, giturl);
    return;
}

fn parse_birthday(args: Vec<String>) {
    fn birthday_help() {
        println!("R2D2 birthday help");
        println!("How to run:");
        println!("   r2d2 birthday COMMAND PARAMETERS");
        println!("\nCommands:");
        println!("   add NAME DATE | add a persons birthday to the list");
        println!("   remove NAME   | remove a person from the list");
        println!("   list          | list all entries");
        println!("   query NAME    | query a name");
        println!("   today         | get all birthdays of today");
        println!("   tomorrow      | get all birthdays of tomorrow");
    }
    if args.len() == 0 {
        println!(
            "{} {}",
            "error:".bold().red(),
            "No arguments given to birthday"
        );
        birthday_help();
        return;
    }
    let request: std::string::String = args[0].clone();

    // Not very clean but Ill keep it for now
    if request.chars().nth(0).unwrap() == '-' && request.chars().nth(1).unwrap() == 'h' {
        birthday_help();
        return;
    }
    match request.as_str() {
        "add" | "a" => {
            if args.len() != 3 {
                println!(
                    "{} {}",
                    "error:".red().bold(),
                    "Wrong amount of arguments given"
                );
                birthday_help();
                return;
            }
            birthday::add_bd(&args[1], &args[2])
        }
        "remove" | "rm" => {
            if args.len() != 2 {
                println!(
                    "{} {}",
                    "error:".red().bold(),
                    "Wrong amount of arguments given"
                );
                birthday_help();
                return;
            }
            birthday::remove_bd(&args[1])
        }
        "list" | "l" => {
            if args.len() != 1 {
                println!(
                    "{} {}",
                    "error:".red().bold(),
                    "Wrong amount of arguments given"
                );
                birthday_help();
                return;
            }
            birthday::list_bd()
        }
        "query" | "q" => {
            if args.len() != 2 {
                println!(
                    "{} {}",
                    "error:".red().bold(),
                    "Wrong amount of arguments given"
                );
                birthday_help();
                return;
            }
            birthday::query_bd(&args[1]);
        }
        "today" => {
            if args.len() != 1 {
                println!(
                    "{} {}",
                    "error:".red().bold(),
                    "Wrong amount of arguments given"
                );
                birthday_help();
                return;
            }
            birthday::today_bd()
        }
        "tomorrow" | "tmrw" => {
            if args.len() != 1 {
                println!(
                    "{} {}",
                    "error:".red().bold(),
                    "Wrong amount of arguments given"
                );
                birthday_help();
                return;
            }
            birthday::tmrw_bd()
        }

        _ => birthday_help(),
    };
}

fn print_ascii_art() {
    //     .-"""-.
    //    /  [O]  \
    //   _|____o__|_
    //  / | +==== | \
    //  |_|  |□|  |_|
    //  | |" |□|  | |
    //  | |L \V/ C| |
    //  | |\_____/| |
    // /__\       /__\
    println!("{}", "    .-\"\"\"-.");

    print!("{}", "   /  ");
    print!("{}", "[".blue());
    //print!("{}", "O".bright_black().bold());
    print!("{}", "O".cyan().bold());
    print!("{}", "]".blue());
    println!("  \\");

    println!("{}", "  _|____o__|_");

    print!("{}", " / | ");
    print!("{}", "+====".blue());
    println!(" | \\");

    print!("{}", " |_|  ");
    print!("{}", "|".blue());
    print!("{}", "□");
    print!("{}", "|".blue());
    println!("  |_|");

    print!("{}", " | |\" ");
    print!("{}", "|".blue());
    print!("{}", "□");
    print!("{}", "|".blue());
    println!("  | |");

    print!("{}", " | |");
    print!("{}", "L ".white().bold());
    print!("{}", "\\".blue());
    print!("{}", "V".white().bold());
    print!("{}", "/".blue());
    print!("{}", " C".white().bold());
    println!("| |");

    println!("{}", " | |\\_____/| |");

    println!("{}", "/__\\       /__\\");
}

extern crate colored;
extern crate getopts;

mod initproj;

use colored::*;
use getopts::Options;
use std::env;

fn print_help() {
    println!("The assistant R2D2!");
    print_ascii_art();
    println!("Usages:");
    println!("    init LANG | init a project for given language (see r2d2 init -h for more info)");
    println!("    task      | Not yet implemented");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", "No arguments given to R2D2".red());
        print_help();
        return;
    }
    match args[1].as_str() {
        "init" | "i" => parse_init(args[2..].to_vec()),
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
    }
    if args.len() == 0 {
        println!("{}", "No arguments given to init".red());
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
            println!("{}", f.to_string().red());
            // println!("{}", "Unrecognized option".red());
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

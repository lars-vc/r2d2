extern crate colored;

use colored::*;
use regex::Regex;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::string::String;

pub fn init_project(lang: String, nvim: bool, giturl: String) {
    match lang.as_str() {
        "python" | "py" => {
            setup_python(nvim);
            setup_git(giturl);
        }
        "rust" => {
            setup_rust(nvim);
            setup_git(giturl)
        }
        "c" => {
            setup_c(nvim);
            setup_git(giturl);
        }
        "c++" | "cpp" => {
            setup_cpp(nvim);
            setup_git(giturl);
        }
        _ => println!("{}", "Language not supported yet (see r2d2 init -h)".red()),
    }
}

fn setup_c(nvim: bool) {
    // Create src folder
    fs::create_dir_all("src/").expect("Couldn't create src dir");

    // Create c file
    let cmainpathstr = get_files_path("c_main")
        .expect("Couldn't resolve file path")
        .into_os_string();
    let cmain: String = fs::read_to_string(cmainpathstr).expect("Couldn't read c_main file");
    fs::write("src/main.c", cmain).expect("Couldn't create c file");

    if nvim {
        create_nvimdebug("c_nvimdebug");
        // TODO clangd format file
    }

    create_gitignore("c_gitignore");

    // Setup make file
    // TODO
    // https://github.com/TheNetAdmin/Makefile-Templates/tree/master/MediumProject
}

fn setup_cpp(nvim: bool) {
    // Create src folder
    fs::create_dir_all("src/").expect("Couldn't create src dir");

    // Create cpp file
    let cppmainpathstr = get_files_path("cpp_main")
        .expect("Couldn't resolve file path")
        .into_os_string();
    let cppmain: String = fs::read_to_string(cppmainpathstr).expect("Couldn't read cpp_main file");
    fs::write("src/main.cpp", cppmain).expect("Couldn't create cpp file");

    if nvim {
        create_nvimdebug("cpp_nvimdebug");
        // TODO clangd format file
    }

    create_gitignore("cpp_gitignore");

    // Setup make file
    // TODO
}

fn setup_rust(nvim: bool) {
    Command::new("cargo")
        .arg("init")
        .status()
        .expect("failed to execute process");
    if nvim {
        // Get project name, this is some advanced rust I dont really get yet
        // let re = Regex::new(r"^.*/(.*)$").unwrap();
        // let pathname_os = env::current_dir()
        //     .expect("Couldn't get current dir")
        //     .into_os_string();
        // let pathname = pathname_os.into_string().expect("Error");
        // let projectname = re.replace(&pathname, "${1}");

        // Make vimspector file and insert project name
        let nvimpathstr = get_files_path("rust_nvimdebug")
            .expect("Couldn't resolve file path")
            .into_os_string();
        let nvimdebug: String = fs::read_to_string(nvimpathstr)
            .expect("Couldn't read rust_nvimdebug file")
            .replace("MYAPP", &get_project_name());
        fs::write(".vimspector.json", nvimdebug).expect("Couldn't create nvimdebug file");
    }
}

fn setup_python(nvim: bool) {
    // Get project name
    let re = Regex::new(r"^.*/(.*)$").unwrap();
    let pathname_os = env::current_dir()
        .expect("Couldn't get current dir")
        .into_os_string();
    let pathname = pathname_os.into_string().expect("Error");
    let projectname = re.replace(&pathname, "${1}").to_string();

    // Create a gitignore
    // let ignorespathstr = get_files_path("python_gitignore")
    //     .expect("Couldn't resolve file path")
    //     .into_os_string();
    // let gitignore: String =
    //     fs::read_to_string(ignorespathstr).expect("Couldn't read python_gitignore file");
    // fs::write(".gitignore", gitignore).expect("Couldn't create gitignore file");
    create_gitignore("python_gitignore");

    // Create src folder
    fs::create_dir_all("src/").expect("Couldn't create src dir");

    // Create python file
    let pymainpathstr = get_files_path("python_main")
        .expect("Couldn't resolve file path")
        .into_os_string();
    let pymain: String = fs::read_to_string(pymainpathstr).expect("Couldn't read python_main file");
    let mut writepymain = String::from("src/");
    writepymain.push_str(projectname.as_str());
    writepymain.push_str(".py");
    fs::write(writepymain, pymain).expect("Couldn't create python file");

    // Create a nvim debug file
    if nvim {
        // let nvimpathstr = get_files_path("python_nvimdebug")
        //     .expect("Couldn't resolve file path")
        //     .into_os_string();
        // let nvimdebug: String =
        //     fs::read_to_string(nvimpathstr).expect("Couldn't read python_nvimdebug file");
        // fs::write(".vimspector.json", nvimdebug).expect("Couldn't create nvimdebug file");
        create_nvimdebug("python_nvimdebug");
    }
}
fn setup_git(giturl: String) {
    print!("Setting up git");
    if giturl == "" {
        println!(" from empty url");
        Command::new("git")
            .arg("init")
            .status()
            .expect("failed to execute process");
        Command::new("git")
            .arg("add")
            .arg(".")
            .status()
            .expect("failed to execute process");
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("initing")
            .status()
            .expect("failed to execute process");
        Command::new("git")
            .arg("branch")
            .arg("-M")
            .arg("root")
            .status()
            .expect("failed to execute process");
    } else {
        println!(" from {}", giturl);
        Command::new("git")
            .arg("init")
            .status()
            .expect("failed to execute process");
        Command::new("git")
            .arg("add")
            .arg(".")
            .status()
            .expect("failed to execute process");
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("initing")
            .status()
            .expect("failed to execute process");
        Command::new("git")
            .arg("branch")
            .arg("-M")
            .arg("root")
            .status()
            .expect("failed to execute process");
        Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(giturl)
            .status()
            .expect("failed to execute process");
        Command::new("git")
            .arg("push")
            .arg("-u")
            .arg("origin")
            .arg("root")
            .status()
            .expect("failed to execute process");
    }
}

fn get_files_path(endpath: &str) -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("src");
    dir.push("files");
    dir.push(endpath);
    Ok(dir)
}

fn get_project_name() -> String {
    let re = Regex::new(r"^.*/(.*)$").unwrap();
    let pathname_os = env::current_dir()
        .expect("Couldn't get current dir")
        .into_os_string();
    let pathname = pathname_os.into_string().expect("Error");
    return re.replace(&pathname, "${1}").to_string();
}

fn create_gitignore(langignorefile: &str) {
    // Create a gitignore
    let ignorespathstr = get_files_path(langignorefile)
        .expect("Couldn't resolve file path")
        .into_os_string();
    let gitignore: String =
        fs::read_to_string(ignorespathstr).expect("Couldn't read gitignore file");
    fs::write(".gitignore", gitignore).expect("Couldn't create gitignore file");
}

fn create_nvimdebug(langdebugfile: &str) {
    let nvimpathstr = get_files_path(langdebugfile)
        .expect("Couldn't resolve file path")
        .into_os_string();
    let nvimdebug: String =
        fs::read_to_string(nvimpathstr).expect("Couldn't read python_nvimdebug file");
    fs::write(".vimspector.json", nvimdebug).expect("Couldn't create nvimdebug file");
}

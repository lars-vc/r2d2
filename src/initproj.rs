use std::fs;
use std::process::Command;
use std::string::String;

pub fn init_project(lang: String, nvim: bool, giturl: String) {
    match lang.as_str() {
        "python" | "py" => {
            setup_python(nvim);
            // setup_git(giturl)
        }
        _ => println!("Language not supported yet"),
    }
}

fn setup_python(nvim: bool) {
    let gitignore: String =
        fs::read_to_string("src/ignores/python_gitignore").expect("Couldn't read file");
    fs::write(".gitignoretest", gitignore).expect("Couldn't create file");
    // Command::new("echo")
    //     .arg("hello from setup_python")
    //     .status()
    //     .expect("failed to execute process");
}

fn setup_git(giturl: String) {
    if giturl == "" {
        Command::new("git").arg("init");
        Command::new("git").arg("commit").arg("-m").arg("initing");
        Command::new("git").arg("branch").arg("-M").arg("root");
    } else {
        Command::new("git")
            .arg("init")
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

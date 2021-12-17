use std::process::Command;
use std::string::String;

pub fn init_project(lang: String, nvim: bool, giturl: String) {
    match lang.as_str() {
        "python" | "py" => setup_python(nvim),
        _ => println!("Language not supported yet"),
    }
    println!("{}", "executing git stuff");
    if giturl == "" {
        Command::new("git").arg("init");
        Command::new("git").arg("commit").arg("-m").arg("initing");
        Command::new("git").arg("branch").arg("-M").arg("root");
    } else {
        Command::new("git")
            .arg("init")
            .status()
            .expect("failed to execute process");

        // Command::new("git")
        //     .arg("commit")
        //     .arg("-m")
        //     .arg("initing")
        //     .status()
        //     .expect("failed to execute process");
        // Command::new("git")
        //     .arg("branch")
        //     .arg("-M")
        //     .arg("root")
        //     .status()
        //     .expect("failed to execute process");
        // Command::new("git")
        //     .arg("remote")
        //     .arg("add")
        //     .arg("origin")
        //     .arg(giturl)
        //     .status()
        //     .expect("failed to execute process");
        // Command::new("git")
        //     .arg("push")
        //     .arg("-u")
        //     .arg("origin")
        //     .arg("root")
        //     .status()
        //     .expect("failed to execute process");
    }
}

fn setup_python(nvim: bool) {
    Command::new("echo")
        .arg("hello from setup_python")
        .status()
        .expect("failed to execute process");
}

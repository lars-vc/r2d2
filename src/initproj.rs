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
        _ => println!("Language not supported yet"),
    }
}

fn setup_rust(nvim: bool) {
    Command::new("cargo")
        .arg("init")
        .status()
        .expect("failed to execute process");
    if nvim {
        // Get project name, this is some advanced rust I dont really get yet
        let re = Regex::new(r"^.*/(.*)$").unwrap();
        let pathname_os = env::current_dir()
            .expect("Couldn't get current dir")
            .into_os_string();
        let pathname = pathname_os.into_string().expect("Error");
        let projectname = re.replace(&pathname, "${1}");

        // Make vimspector file and insert project name
        let nvimpathstr = get_files_path("rust_nvimdebug")
            .expect("Couldn't resolve file path")
            .into_os_string();
        let nvimdebug: String = fs::read_to_string(nvimpathstr)
            .expect("Couldn't read rust_nvimdebug file")
            .replace("MYAPP", &projectname);
        fs::write(".vimspector.json", nvimdebug).expect("Couldn't create nvimdebug file");
    }
}

fn setup_python(nvim: bool) {
    // Create a gitignore
    let ignorespathstr = get_files_path("python_gitignore")
        .expect("Couldn't resolve file path")
        .into_os_string();
    let gitignore: String =
        fs::read_to_string(ignorespathstr).expect("Couldn't read python_gitignore file");
    fs::write(".gitignore", gitignore).expect("Couldn't create gitignore file");

    // Create src folder
    fs::create_dir_all("src/").expect("Couldn't create src dir");

    // Create a nvim debug file
    if nvim {
        let nvimpathstr = get_files_path("python_nvimdebug")
            .expect("Couldn't resolve file path")
            .into_os_string();
        let nvimdebug: String =
            fs::read_to_string(nvimpathstr).expect("Couldn't read python_nvimdebug file");
        fs::write(".vimspector.json", nvimdebug).expect("Couldn't create nvimdebug file");
    }
    // Command::new("echo")
    //     .arg("hello from setup_python")
    //     .status()
    //     .expect("failed to execute process");
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

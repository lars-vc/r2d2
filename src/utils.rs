use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn get_files_path(endpath: &str) -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("files");
    dir.push(endpath);
    Ok(dir)
}

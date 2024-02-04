use std::env::current_dir;
use std::fs;
use std::fs::{create_dir_all, File};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

pub fn get_current_path() -> Result<PathBuf, Error> {
    current_dir()
}

pub fn create_dir(path_buf: PathBuf, dir_name: &str) -> Result<String, Error>{

    match create_dir_all(path_buf.join(dir_name)) {
        Ok(dir_path) => Ok(format!("Success! {dir_path:?}")),
        Err(e) => Err(e)
    }
}
pub fn create_file(path_buf: &Path, file_name: &str) -> Result<File, Error>{
    File::create(path_buf.join(file_name))
}

pub fn write_into_file(mut file: File, msg: String) {
    file.write_all(msg.as_ref()).expect("idk how to write");
}

pub fn delete_file(path_buf: &Path) {
    fs::remove_file(path_buf).expect("idk how to delete");
}
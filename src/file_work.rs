use std::env::current_dir;
use std::fs;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

pub fn get_current_path() -> Result<PathBuf, Error> {
    current_dir()
}

pub fn create_dir(path_buf: PathBuf, dir_name: &str) -> Result<(), Error>{

    match create_dir_all(path_buf.join(dir_name)) {
        Ok(dir_path) => Ok(dir_path),
        Err(e) => Err(e)
    }
}
pub fn create_file(path: &Path, file_name: &str) -> Result<File, Error>{
    File::create(path.join(file_name))
}

pub fn write_into_file(path: &Path, msg: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path).expect("Failed to open the file");
    
    file.write_all(msg.as_ref()).expect("idk how to write");
}

pub fn delete_file(path_buf: &Path) {
    fs::remove_file(path_buf).expect("idk how to delete");
}

pub fn dir(path_buf: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path_buf)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.

    Ok(entries)
}

pub fn read_file(path_buf: &Path) -> Result<String, Error>{
    fs::read_to_string(path_buf)
}

pub fn export_into_json(string: String) {
    let curr_dir = get_current_path().expect("Can't get current dir!");
    if create_dir(curr_dir.clone(), "saved_works").is_ok() {
        let file_path = curr_dir.join("saved_works").join("saved.json");
        write_into_file(file_path.as_path(), string);
    };
}
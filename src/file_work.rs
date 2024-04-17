use std::env::current_dir;
use std::fs;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

pub fn get_current_path_buf() -> PathBuf {
    current_dir().expect("Failed to get current dir")
}

pub fn create_dir(path_buf: PathBuf, dir_name: &str) {
    create_dir_all(path_buf.join(dir_name)).expect("Failed to create a new dir")
}

#[allow(dead_code)]
pub fn create_file(path: &Path, file_name: &str) -> File {
    File::create(path.join(file_name)).expect("Failed to create a new file")
}

pub fn file_empty_then_write(path: &Path, msg: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open the file");

    file.set_len(0).expect("Failed to empty the file");
    file.flush().expect("Failed to flush the file");
    file.write_all(msg.as_ref()).expect("idk how to write");
}
#[allow(dead_code)]
pub fn delete_file(path_buf: &Path) {
    fs::remove_file(path_buf).expect("idk how to delete");
}

pub(crate) fn dir(path_buf: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path_buf)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.

    Ok(entries)
}

pub fn read_file(path_buf: &Path) -> String {
    fs::read_to_string(path_buf).expect("Failed to read the file")
}

pub fn export_into_json(string: String) {
    let curr_dir = get_current_path_buf();
    create_dir(curr_dir.clone(), "saved_works");
    let file_path = curr_dir.join("saved_works/").join("saved.json");

    file_empty_then_write(file_path.as_path(), string);
}

pub fn get_export_json_loc() -> PathBuf {
    let curr_dir = get_current_path_buf();
    create_dir(curr_dir.clone(), "saved_works");
    curr_dir.join("saved_works/").join("saved.json")
}

use std::{
    io,
    path::{Path, PathBuf},
};

use crate::models::{FileType, Item};

pub fn create_item(path: &Path, file_type: &FileType) -> Result<String, io::Error> {
    todo!("create");
}

pub fn move_item(path: &Path, file_type: &FileType) -> Result<String, io::Error> {
    todo!("move");
}
pub fn copy_item(path: &Path, file_type: &FileType) -> Result<String, io::Error> {
    todo!("copy");
}
pub fn rename_item(path: &Path, new_name: &str, file_type: &FileType) -> Result<String, io::Error> {
    todo!("rename");
}

pub fn delete_item(path: &Path, file_type: &FileType) -> Result<String, io::Error> {
    todo!("move");
}

pub fn dir(path: &Path) -> Result<Vec<Item>, io::Error> {
    todo!("move");
}
pub fn get_file_contents(path: &Path) -> Result<String, io::Error> {
    todo!("move");
}

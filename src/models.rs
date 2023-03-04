pub enum FileType {
    File,
    Folder,
}

pub struct Item {
    name: String,
    file_type: FileType,
    size: u64,
    path: String,
}

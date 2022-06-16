pub struct FileInfoData {
    directory: String,
    name: String,
    location: String,
}

impl FileInfoData {
    pub fn new(
        directory: &'static str,
        name: &'static str,
        location: &'static str,
    ) -> FileInfoData {
        FileInfoData {
            directory: String::from(directory),
            name: String::from(name),
            location: String::from(location),
        }
    }
}

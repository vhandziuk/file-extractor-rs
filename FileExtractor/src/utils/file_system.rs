use std::fs;
use std::io;
use std::path;
use walkdir::WalkDir;

pub trait IFileSystem {
    fn directory_exists(&self, path: &String) -> bool;
    fn create_directory(&self, path: &String) -> io::Result<()>;
    fn file_exists(&self, path: &String) -> bool;
    fn get_files(&self, path: &String, extension: &String) -> Vec<String>;
}

pub struct FileSystem;

impl IFileSystem for FileSystem {
    fn directory_exists(&self, path: &String) -> bool {
        path::Path::new(path).is_dir()
    }
    fn create_directory(&self, path: &String) -> io::Result<()> {
        fs::create_dir(path)
    }
    fn file_exists(&self, path: &String) -> bool {
        path::Path::new(path).exists()
    }
    fn get_files(&self, path: &String, extension: &String) -> Vec<String> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Ok(fs_entry) => {
                    if let Some(file_name) = fs_entry.file_name().to_str() {
                        if file_name.to_lowercase().ends_with(extension) {
                            match fs_entry.into_path().into_os_string().into_string() {
                                Ok(file_path) => Some(file_path),
                                Err(_) => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Err(_) => None,
            })
            .collect::<Vec<_>>()
    }
}

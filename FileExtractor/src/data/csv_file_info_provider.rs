use super::FileInfoData;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait ICsvFileInfoProvider {
    fn enumerate_entries(&self, path: &String) -> Vec<FileInfoData>;
}

pub struct CsvFileInfoProvider;

impl ICsvFileInfoProvider for CsvFileInfoProvider {
    fn enumerate_entries(&self, path: &String) -> Vec<FileInfoData> {
        // if let Ok(lines) = read_lines(path.as_str()) {
        //     let file_infos = lines
        //         .filter_map(|line| match line {
        //             Ok(l) => {
        //                 let splitted = l.split(',').collect::<Vec<_>>();
        //                 Some(FileInfoData {
        //                     name: String::from(splitted[0]),
        //                     directory_name: String::from(splitted[1]),
        //                 })
        //             }
        //             Err(_) => None,
        //         })
        //         .collect::<Vec<_>>();

        //     file_infos
        //         .into_iter()
        //         .for_each(|b| info!("{} {}", b.name, b.directory_name));
        // };
        todo!()
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

mod sync;
use crate::data;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
pub mod blacklist;
pub mod whitelist;
pub mod ip;
pub mod port;

fn read_lines(file_path: String) -> Vec<String> {

    let file = File::open(file_path).unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|l| l.ok()).collect();

    return lines;

}

pub fn sync_plugins() {

    let repos: Vec<String> = read_lines(data::path::sync_file());

    thread::spawn(|| {
        let _ = sync::sync(repos, data::path::sync_dir());
    });

}

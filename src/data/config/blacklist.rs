use once_cell::sync::Lazy;
use crate::data;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};

static BLACKLIST: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| {
    let bl = Arc::new(Mutex::new(read_lines(data::path::blacklist_file())));
    bl
});

fn read_lines(file_path: String) -> Vec<String> {

    let file = File::open(file_path).unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|l| l.ok()).collect();

    return lines;

}

pub fn get(refresh: bool) -> Vec<String> {

    if refresh {
        let mut lines: Vec<String> = read_lines(data::path::blacklist_file());
        let mut lock = BLACKLIST.lock().unwrap();
        lock.clear();
        lock.append(&mut lines);
    }

    return BLACKLIST.lock().unwrap().to_vec();

}

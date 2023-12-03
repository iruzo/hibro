use once_cell::sync::Lazy;
use crate::data;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};

static PORT: Lazy<Arc<Mutex<String>>> = Lazy::new(|| {
    let port = Arc::new(Mutex::new(get(false)));
    port
});

fn read_lines(file_path: String) -> Vec<String> {

    let file = File::open(file_path).unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|l| l.ok()).collect();

    return lines;

}

pub fn get(refresh: bool) -> String {

    if refresh {
        let lines = read_lines(data::path::config_file());
        let mut port = String::new();
        for line in lines.iter() {
            if line.contains("port: ") {
                port.clone_from(&line.split(" ").last().unwrap().to_string());
            }
        }
        PORT.lock().unwrap().clone_from(&port);
    }

    return PORT.lock().unwrap().to_string();

}


pub fn set(port: &String) {
    PORT.lock().unwrap().clone_from(port);
}

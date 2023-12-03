use std::fs;
use crate::data;
use std::path::PathBuf;

/// Return all file paths inside the directory and its subdirectories.
fn dir_paths(dir_path: String) -> Vec<std::string::String> {
    let paths = fs::read_dir(dir_path).unwrap();
    let mut all_paths: Vec<std::string::String> = Vec::new();
    for path in paths {
        if path.as_ref().unwrap().path().is_dir() {
            all_paths.append(&mut dir_paths(String::from(path.as_ref().unwrap().path().to_str().unwrap())));
        } else {
            all_paths.push(String::from(path.unwrap().path().to_str().unwrap()));
        }

    }
    return all_paths;
}

/// Represent the connections data tree.
/// This will return all the paths to the files where data from the given connections is stored.
pub fn exec(connections: &Vec<&data::model::connection::Connection>) -> Vec<std::string::String> {

    // obtain all file paths
    let mut data_tree: Vec<std::string::String> = dir_paths(data::path::connections_dir());

    // filter by given connections
    data_tree.retain(|s| connections.iter().any(|t| s.contains(&t.fingerprint)));

    // remove path and get only file names
    let mut data_tree_file_names: Vec<std::string::String> = Vec::new();
    for data in data_tree {
        let pb = PathBuf::from(data);
        data_tree_file_names.push(String::from(pb.file_name().unwrap().to_str().unwrap()));
    }

    return data_tree_file_names;

}

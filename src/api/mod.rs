use crate::harbor::websocket;
use crate::data;
use std::path::PathBuf;
use std::fs;

/// send data to a list of connections
pub fn send(fingerprints: &Vec<String>, message: &String) {

    for con in data::mem::CONNECTIONS.lock().unwrap().iter() {
        if fingerprints.contains(&con.fingerprint) {
            // websocket
            if con.ws_sender.is_some() {
                websocket::send(con, &message);
            }
        }
    }

}

/// return all connections as JSON
/// {
/// "fingerprint": "ip",
/// "fingerprint": "ip",
/// "fingerprint": "ip",
/// ...
/// }
pub fn connections() -> String {
    let connections = data::mem::CONNECTIONS.as_ref().lock().unwrap();
    let mut json: String = String::from("{");

    if connections.len() != 0 {

        for con in connections.iter() {
            json.push_str("\"");
            json.push_str(&con.fingerprint);
            json.push_str("\": \"");
            json.push_str(&con.ip);
            json.push_str("\",");
        }
        json = json.strip_suffix(",").unwrap().to_string();

    }

    json.push_str("}");

    return json
}

/// return all data_tree from a list of connections
/// {
/// "fingerprint": ["data_file_name1", "data_file_name2", "data_file_name3"],
/// "fingerprint": ["data_file_name1", "data_file_name2", "data_file_name3"],
/// "fingerprint": ["data_file_name1", "data_file_name2", "data_file_name3"],
/// ...
/// }
pub fn connections_data_tree(connections: &Vec<&data::model::connection::Connection>) -> String {
    let mut json: String = String::from("{");

    if !connections.is_empty() {
        for con in connections {
            let mut vec_con = Vec::new();
            vec_con.push(con.clone());
            let data_tree = data::data_tree::exec(&vec_con);

            json.push_str("\"");
            json.push_str(&con.fingerprint);
            json.push_str("\": [");
            if !data_tree.is_empty() {
                for data in data_tree {
                    json.push_str("\"");
                    json.push_str(&data);
                    json.push_str("\",");
                }
                json = json.strip_suffix(",").unwrap().to_string();
            }
            json.push_str("],");
        }
        json = json.strip_suffix(",").unwrap().to_string();
    }
    json.push_str("}");

    return json;
}

/// return data from specified file
pub fn connection_data_value(con: &data::model::connection::Connection, file_name: &String) -> std::string::String {

    // let mut data_value: std::string::String = path::connections_dir();
    let p = PathBuf::from(data::path::connections_dir())
        .join(&con.ip)
        .join(&con.fingerprint)
        .join(file_name);

    return fs::read_to_string(p).unwrap_or(String::from(""));

}

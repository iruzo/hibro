use crate::harbor;
use crate::data;
use std::path::PathBuf;
use std::fs;
use std::thread;

/// # Open a websocket server
///
/// ## Arguments
/// - &String - ip: ip used to open the websocket server.
///     - e.g. "0.0.0.0"
/// - &String - port: port to open the websocket server.
///     - e.g. "4444"
///
/// ## Return
/// - None
///
/// ## Example
/// ```rust
/// open_websocket("0.0.0.0", "4444")
/// ```
pub fn open_websocket(ip: &String, port: &String) {

    let ip_clone = ip.clone();
    let port_clone = port.clone();

    thread::spawn(move || {
        harbor::websocket::open(&ip_clone, &port_clone);
    });

}

/// # Open a tcp server
///
/// ## Arguments
/// - &String - ip: ip used to open the tcp server.
///     - e.g. "0.0.0.0"
/// - &String - port: port to open the tcp server.
///     - e.g. "4444"
///
/// ## Return
/// - None
///
/// ## Example
/// ```rust
/// open_tcp("0.0.0.0", "4444")
/// ```
pub fn open_tcp(ip: &String, port: &String) {

    let ip_clone = ip.clone();
    let port_clone = port.clone();

    thread::spawn(move || {
        let _ = harbor::tcp::open(&ip_clone, &port_clone);
    });

}

/// # Open an udp server
///
/// ## Arguments
/// - &String - ip: ip used to open the udp server.
///     - e.g. "0.0.0.0"
/// - &String - port: port to open the udp server.
///     - e.g. "4444"
///
/// ## Return
/// - None
///
/// ## Example
/// ```rust
/// open_udp("0.0.0.0", "4444")
/// ```
pub fn open_udp(ip: &String, port: &String) {

    let ip_clone = ip.clone();
    let port_clone = port.clone();

    thread::spawn(move || {
        let _ = harbor::udp::open(&ip_clone, &port_clone);
    });

}

/// # Send data to a list of connections
///
/// ## Arguments
/// - &Vec<String> - fingerprints: List of fingerprints to send data to.
/// - &String - message: Message that will be send to those fingerprints.
///
/// ## Return
/// - None
///
/// ## Example
/// ```rust
/// let mut fingerprints: Vec<String> = Vec::new();
/// for string in strings.iter() {
///     fingerprints.push(string);
/// }
/// send(&fingerprints, &String::from("Your message"));
/// ```
pub fn send(fingerprints: &Vec<String>, message: &String) {

    for con in data::mem::CONNECTIONS.lock().unwrap().iter() {
        if fingerprints.contains(&con.fingerprint) {
            if con.ws_sender.is_some() {
                harbor::websocket::send(con, &message);
            }
        }
    }

}

/// # Return all connections as JSON
///
/// ## Arguments
/// - None
///
/// ## Return
/// - JSON as String
/// ```
/// {
/// "fingerprint": "ip",
/// "fingerprint": "ip",
/// "fingerprint": "ip",
/// ...
/// }
/// ```
///
/// ## Example
/// ```rust
/// connections()
/// ```
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

/// # Return all data_tree from a list of connections
///
/// ## Arguments
/// - &Vec<&data::model::connection::Connection> - connections: Reference to a Vector of
/// Connections.
///
/// ## Return
/// - JSON as String
///
/// ## Example
/// ```
/// {
/// "fingerprint": ["data_file_name1", "data_file_name2", "data_file_name3"],
/// "fingerprint": ["data_file_name1", "data_file_name2", "data_file_name3"],
/// "fingerprint": ["data_file_name1", "data_file_name2", "data_file_name3"],
/// ...
/// }
/// ```
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

/// # Return data from specified file
///
/// ## Arguments
/// - &data::model::connection::Connection - con: Reference to connection object.
/// - &String - file_name: File name.
///
/// ## Return
/// - String
///
/// ## Example
/// ```rust
/// connection_data_value(connection, &String::from("filename"))
/// ```
pub fn connection_data_value(con: &data::model::connection::Connection, file_name: &String) -> std::string::String {

    // let mut data_value: std::string::String = path::connections_dir();
    let p = PathBuf::from(data::path::connections_dir())
        .join(&con.ip)
        .join(&con.fingerprint)
        .join(file_name);

    return fs::read_to_string(p).unwrap_or(String::from(""));

}

pub mod tcp;
pub mod udp;
pub mod websocket;
use crate::data;
use std::path::PathBuf;
use std::sync::MutexGuard;
use std::thread;
use substring::Substring;
use data::model::connection::Connection;

fn process_message(mut connections: MutexGuard<Vec<Connection>>, connection: &data::model::connection::Connection, message: &String) {

    let mut new_fingerprint = "undefined".to_string();

    if message.contains("fingerprint") {
        let start = message.find("fingerprint\": \"").unwrap_or(0);
        let end = message.find("\",").unwrap_or(message.len());
        new_fingerprint = message.substring(start, end).split(": \"").last().unwrap().split("\"").take(1).last().unwrap().to_string();
        if &new_fingerprint != &connection.fingerprint {

            let fingerprint_clone = connection.fingerprint.clone();
            let new_fingerprint_clone = new_fingerprint.clone();

            // replace fingerprint in connection in array data::mem::CONNECTIONS with the new_fingerprint
            let mut i = 0;
            for con in connections.iter() {
                if con.ws_sender.clone().unwrap().connection_id() == connection.ws_sender.clone().unwrap().connection_id() {
                    break;
                }
                i += 1;
            }
            connections.swap_remove(i);
            connections.push(data::model::connection::Connection{
                su: connection.su.clone(),
                ip: connection.ip.clone(),
                ws_sender: connection.ws_sender.clone(),
                fingerprint: new_fingerprint.clone()
            });

            let mut source_folder = PathBuf::new();
            source_folder.push(data::path::connections_dir());
            source_folder.push(connection.ip.clone());
            source_folder.push(fingerprint_clone);

            let mut destination_folder = PathBuf::new();
            destination_folder.push(data::path::connections_dir());
            destination_folder.push(connection.ip.clone());
            destination_folder.push(new_fingerprint_clone);

            thread::spawn(move || {
                if let Err(_err) = data::move_with_delete::exec(source_folder.to_str().unwrap(), destination_folder.to_str().unwrap()) {
                    println!("Failer to move data from old fingerprint folder");
                    println!("{}", _err.to_string());
                }
            });

        }
    }

    if new_fingerprint == "undefined" {
        new_fingerprint = connection.fingerprint.clone();
    }

    let ip_clone: String = connection.ip.clone();
    let message_clone: String = message.clone();

    // save message on file
    thread::spawn(move || {
        let _ = data::save::exec(data::path::connections_dir(), ip_clone, message_clone, new_fingerprint);
    });
}

use base64::{Engine as _, engine::general_purpose};
use crate::data;
use data::model::connection::Connection;
use minify_js::{Session, TopLevelMode, minify};
use rand::Rng;
use std::println;
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};

use super::process_message;

struct Server {
    server_sender: Sender,
}

impl Handler for Server {

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {

        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(100000000..=999999999);

        data::mem::CONNECTIONS.lock().unwrap().push(Connection{
            su: false,
            ip: Some(handshake.remote_addr()?).unwrap().unwrap(),
            ws_sender: Some(self.server_sender.clone()),
            fingerprint: random_number.to_string()
        });
        println!("{}", Some(handshake.remote_addr()?).unwrap().unwrap());

        Ok(())

    }

    /// Handle messages that comes from the websocket connection
    fn on_message(&mut self, message: Message) -> Result<()> {

        // let connections = data::mem::CONNECTIONS.lock().unwrap();
        // for con in connections.iter() {
        //     if con.ws_sender.clone().unwrap().connection_id() == self.server_sender.connection_id() {
        //         process_message(connections, con, &message.to_string());
        //         break;
        //     }
        // }


        let connections = data::mem::CONNECTIONS.lock().unwrap();
        let connections_clone = connections.clone();
        for con in connections_clone.iter() {
            if con.ws_sender.clone().unwrap().connection_id() == self.server_sender.connection_id() {
                process_message(connections, con, &message.to_string());
                break;
            }
        }

        Ok(())

    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {

        let mut i = 0;
        let mut connections = data::mem::CONNECTIONS.lock().unwrap();
        for con in connections.iter() {
            if con.ws_sender.clone().unwrap().connection_id() == self.server_sender.connection_id() {
                break;
            }
            i += 1;
        }
        connections.swap_remove(i);
        println!("closed connection");

    }
}

/// Open a websocket and manage every connection on the given list
pub fn open(url: &str, port: &str) {

    listen(format!("{url}:{port}"), |sender| {
        Server {
            server_sender: sender
        }
    }).unwrap();

}

/// send data to a list of connections
pub fn send(connection: &Connection, message: &String) {

    let session = Session::new();
    let mut out = Vec::new();
    minify(&session, TopLevelMode::Global, message.as_bytes(), &mut out).unwrap();
    let _ = connection.ws_sender.as_ref().unwrap().send(general_purpose::STANDARD.encode(out.as_slice()));

}

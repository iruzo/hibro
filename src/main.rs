// mod pool;
// use clap::{Command, Arg, crate_version, crate_authors, crate_description };
//use hibro::path::create;
mod api;
mod harbor;
mod data;
use clap::Parser;
use core::time::Duration;
use std::io::{stdin, stdout, Write};
use std::{thread, println};

/// C2
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// tui
    #[arg(short, long)]
    ui: bool,

    /// run the daemon
    #[arg(short, long)]
    d: bool,

    /// kill the daemon
    #[arg(short, long)]
    e: bool,

    /// Do not save any data at disk
    #[arg(short, long)]
    memfilesys: bool,

    /// ip
    #[arg(short, long, default_value_t = String::from(""))]
    ip: String,

    /// port
    #[arg(short, long, default_value_t = String::from(""))]
    port: String,

}

fn argparser() {

    let args = Args::parse();

    if args.ui == true {
    	println!("UI: {:?}", args.ui);
    }

    if args.d == true {
    	println!("daemon: {:?}", args.ui);
    }

    if args.e == true {
        std::process::exit(0x0100);
    }

    if args.memfilesys == true {
        println!("in memory file system: {:?}", args.memfilesys);
    }

    if !args.ip.is_empty() {
        data::config::ip::set(&args.ip);
    }

    if !args.port.is_empty() {
        data::config::port::set(&args.port);
    }

}

fn main() {
    // test_websocket();
    // argparser();

    // let _ = harbor::udp::open("0.0.0.0", "4444");
    // let args: Vec<String> = std::env::args().collect();    let args: Vec<String> = std::env::args().collect();
}

fn test_data_tree() {
    let mut cons: Vec<&data::model::connection::Connection> = Vec::new();
    let connections = data::mem::CONNECTIONS.as_ref().lock().unwrap();
    for con in connections.iter() {
        cons.push(con);
    }
    println!("{}", api::connections_data_tree(&cons));
}

// only test with a single value from connection
fn test_data_value() {
    let connections = data::mem::CONNECTIONS.as_ref().lock().unwrap();
    let mut cons: Vec<&data::model::connection::Connection> = Vec::new();
    for con in connections.iter() {
        cons.push(con);
        let data_tree = api::connections_data_tree(&cons);

        let mut start_bytes = data_tree.find(": [\"").unwrap_or(0);
        start_bytes = start_bytes + 4;
        let end_bytes = data_tree.rfind("\"").unwrap_or(data_tree.len());
        let result = &data_tree[start_bytes..end_bytes];

        println!("{}", api::connection_data_value(con, &String::from(result)));
        cons.clear();
    }
}

fn test_sync() {
    data::config::sync_plugins();
    thread::sleep(Duration::from_millis(5000))
}

fn test_connections() {
    println!("{}", api::connections());
}

fn test_websocket() {

    // open a web socket
    thread::spawn(|| {
        let _ = harbor::websocket::open("0.0.0.0", "4444");
    });

    loop {
        print!("command: ");
        stdout().flush().expect("Error flushing stdout");
        let mut response = String::new();
        stdin().read_line(&mut response).expect("Error reading line from stdin");
        let response = response.trim();

        if response == "connections" {

            test_connections();

        } else if response.contains("send")  {

            let mut fingerprints: Vec<String> = Vec::new();
            for con in data::mem::CONNECTIONS.lock().unwrap().iter() {
                fingerprints.push(con.fingerprint.clone());
            }

            api::send(&fingerprints, &String::from("const main = () => { let my_first_variable = 1; };"));

        } else if response.contains("datatree")  {

            test_data_tree();

        } else if response.contains("datavalue")  {

            test_data_value();

        } else if response.contains("exit") {

            std::process::exit(0x0100);

        }

    }
}

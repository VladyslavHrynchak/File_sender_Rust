mod client;
use client::Client;

use std::{str::FromStr, io};
fn main() {
    loop {
        let mut path_to_file = String::new();
        println!("Enter file path: ");
        io::stdin().read_line(&mut path_to_file).unwrap();
        let path_to_file = path_to_file.trim().to_string();

        let mut client = Client::new();
        client.start(String::from_str("127.0.0.1").unwrap(), 7575);

        client.send_file(&path_to_file).unwrap_or_else(|e|{
            println!("{e}");
        })
    }
    
}

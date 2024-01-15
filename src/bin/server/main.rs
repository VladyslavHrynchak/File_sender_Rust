mod server;
use server::Server;

use std::str::FromStr;
fn main() {
    let mut server = Server::new();
    server.start(String::from_str("0.0.0.0").unwrap(), 7575);  
}

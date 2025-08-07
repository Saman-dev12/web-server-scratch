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
	let mut buffer = [0; 2048];
    	stream.read(&mut buffer).await?;

	let request_str = String::from_utf8_lossy(&buffer);
	let mut parts = request_str.lines().next().unwrap_or("").split_whitespace();
	let method = parts.next().unwrap_or("GET");
	let path = parts.next().unwrap_or("/");

    
        println!("Method : {:?}",method);
        println!("Path : {:?}",path);
	//println!("{:?}",stream);
	let res = String::from("HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!");
	stream.write_all(res.as_bytes()).await;
	stream.flush().await?;
	Ok(())
}

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs;
use std::path::{Path, PathBuf};
use std::error::Error;

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

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).await?;

    let request_str = String::from_utf8_lossy(&buffer);
    let mut parts = request_str.lines().next().unwrap_or("").split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");

    
    match (method, path) {
        ("GET", "/") => serve_static_file(&mut stream, "src/public/index.html").await?,
        ("GET", path) => serve_static_file(&mut stream, &format!("src/public{}", path)).await?,

        ("POST", "/form") => {
            if let Some(body_start) = request_str.find("\r\n\r\n") {
                let body = &request_str[body_start + 4..];
                println!("📝 Received form data: {}", body);
            }

            let response_body = "<h1>Thank you for your submission!</h1>";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            stream.write_all(response.as_bytes()).await?;
        },

        _ => {
            let response_body = tokio::fs::read("src/public/404.html").await
                .unwrap_or_else(|_| b"<h1>404 Not Found</h1>".to_vec());

            let response = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n",
                response_body.len()
            );

            stream.write_all(response.as_bytes()).await?;
            stream.write_all(&response_body).await?;
        }
    };

    stream.flush().await?;
    Ok(())
}

async fn serve_static_file(stream: &mut TcpStream, file_path: &str) -> Result<(), Box<dyn Error>> {
    let public_path = fs::canonicalize("src/public").await?;
    let requested_path = fs::canonicalize(file_path).await.unwrap_or_else(|_| public_path.clone());

    let (status_line, content, content_type) = if requested_path.starts_with(public_path) && Path::new(file_path).exists() {
        let content = fs::read(file_path).await?;
        let content_type = get_mime_type(file_path);
        ("HTTP/1.1 200 OK", content, content_type)
    } else {
        let content = fs::read("src/public/404.html").await.unwrap_or_else(|_| b"404 Not Found".to_vec());
        ("HTTP/1.1 404 NOT FOUND", content, "text/html")
    };

    let headers = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
        status_line,
        content.len(),
        content_type
    );

    stream.write_all(headers.as_bytes()).await?;
    stream.write_all(&content).await?;

    Ok(())
}

fn get_mime_type(path: &str) -> &str {
    if path.ends_with(".html") { "text/html" }
    else if path.ends_with(".css") { "text/css" }
    else if path.ends_with(".js") { "application/javascript" }
    else if path.ends_with(".png") { "image/png" }
    else if path.ends_with(".jpg") || path.ends_with(".jpeg") { "image/jpeg" }
    else if path.ends_with(".svg") { "image/svg+xml" }
    else { "application/octet-stream" }
}

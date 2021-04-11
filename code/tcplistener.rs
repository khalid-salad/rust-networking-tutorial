fn handle_client(stream: TcpStream) {
    // ...
}

fn main() -> Result<()> {
    let address = "127.0.0.1:80";
    let listener = TcpListener::bind(address)?;

    // accept connections and process serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
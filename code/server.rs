let listener = TcpListener::bind(format!("localhost:{}", port))?;

// accept connections and process them serially
for stream in listener.incoming() {
    handle_client(stream?)?;
}
Ok(())


































//
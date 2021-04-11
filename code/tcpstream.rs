fn main() -> Result<()> {
    let addr = "127.0.0.1:34254"
    let mut stream = TcpStream::connect(addr)?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here
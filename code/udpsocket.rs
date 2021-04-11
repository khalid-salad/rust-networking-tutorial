fn main() -> Result<()> {
  {
    let addr = "127.0.0.1:34254"
    let mut socket = UdpSocket::bind(addr)?;

    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf)?;

    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src)?;
  } // the socket is closed here
  Ok(())
}
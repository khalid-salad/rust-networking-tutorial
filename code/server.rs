let args: Vec<String> = env::args().collect();
match args.len() {
  1 => panic!("Please pass port number to command line."),
  _ => (),
}
let port = &args[1];
let address = format!("localhost:{}", port);
let listener = TcpListener::bind(address)?;
match listener {
  Ok(v) => (),
  Err(e) => panic!("Error connecting: {}", e)
}


















//
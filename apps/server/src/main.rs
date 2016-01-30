use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::io::Write;
use std::io::Read;
use std::io::{BufReader, BufRead};
use std::iter::Iterator;

mod server;

enum MessageType {
    HELLO,
    VERSION,
	QUIT,
	UNKNOWN
}

fn build_message(data: &Vec<u8>) -> MessageType {
	// Take the first 4 characters and clone them into header_bytes
    let header_bytes: Vec<u8> = data.iter().take(4).cloned().collect::<_>();
    
	let header = String::from_utf8(header_bytes).unwrap();
	
	let message_length = data.len();
	
	println!("Size of rec data : {}", message_length);
	
    for letter in header.chars() {
        println!("{}", letter);
    }
     
	if header.eq("HELL")
	{
		return MessageType::HELLO;
	}
	if header.eq("QUIT")
	{
		return MessageType::QUIT;
	}
	if header.eq("VERS")
	{
		return MessageType::VERSION;
	}
		
	
    return MessageType::UNKNOWN;
}

fn handle_client(mut stream: TcpStream) {
	stream.write(b"Welcome to ServerName.\n").expect("Could not send welcome message to client.");

	loop {
		let write_result: Result<usize, _>;
	
		let mut reader = BufReader::new(stream.try_clone().unwrap());
		
		let mut buf = String::new();

        reader.read_line(&mut buf).expect("Could not read client data.");
		
		println!("Message received : {}", buf);
		
        let msg = build_message(&buf.into_bytes());
		
		match msg {
            MessageType::HELLO => {
                println!("Client HELLO");
                
				write_result = stream.write(b"Hi, this is server.\n");
            }
			MessageType::QUIT => { 
				println!("Client Quitting");
                
				write_result = stream.write(b"Bye !\n");
                
				stream.shutdown(Shutdown::Both).expect("Could not shutdown the socket.");
				
				break;
			}
            MessageType::VERSION => {
                println!("Sending client Version");
                
				write_result = stream.write(b"v0.1.0\n");
            }
			_ => {
				write_result = Ok(0);
			}
		}
		
		if write_result.is_err()
		{
			println!("Problem writing to client stream.");
		}
    }
    
    println!("Client exit.");
}

fn main() {

	let chat_serv = server::ChatServer::create();

    let mut clients = Vec::<TcpStream>::new();

    let listener = TcpListener::bind("127.0.0.1:2222").expect("Could not bind to address.");

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        println!("Client connected.");
    
        match stream {
            Ok(stream) => {
                clients.push(stream.try_clone().expect("Could not clone the client stream."));

                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { println!("Connection error, {}.", e); }
        }
    }

    // close the socket server
    drop(listener);
}
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::thread::{Thread, JoinHandle};

pub struct Client {

ident: String,

}

pub struct ClientThreadGroup {

clients: Vec<Client>

}

impl ClientThreadGroup {
	pub fn new() -> ClientThreadGroup {
		return ClientThreadGroup {

			clients: Vec::<Client>::new()
		};
	}
	
	pub fn start_handling(&self,  stream: &TcpStream) -> JoinHandle<()> {
	
		return thread::spawn(|| {
			
			
			
		});
	
	}
}

pub struct ChatServer {

x: i32,

client_thread_groups: Vec<ClientThreadGroup>,

listener: TcpListener,

}

impl ChatServer {

	// TODO : Return Result<ChatServer, _>
	pub fn create() -> ChatServer {
		return ChatServer {
			x: 0,
			client_thread_groups: Vec::<ClientThreadGroup>::new(),
			listener: TcpListener::bind("127.0.0.1:2222").expect("Could not bind to address.") // Temp value
		};
	}
	
	pub fn start(&self) {
		
		let first_client_group = ClientThreadGroup::new();
		
		self.client_thread_groups.push(first_client_group);
		
		for stream in self.listener.incoming() {
			println!("Client connected !");
			
			
		
		}
		
	}

}

// Start
// Close

// HandleClient

// OnReceiveClientMessage
// BroadcastClientMessage

// GetSupportedVersions
// GetLatestSupportedVersion

// ServerTick
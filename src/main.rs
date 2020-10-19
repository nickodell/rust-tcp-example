use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::io::Error;

const BUFSIZE: usize = 128;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
	// Get address of remote end
	let peer_addr = stream.peer_addr()?;
	let mut data = [0 as u8; BUFSIZE];
	let mut keep_going = true;
	while keep_going {
		keep_going = match stream.read(&mut data) {
			Ok(size) => {
				// echo everything!
				stream.write(&data[0..size])?;
				true
			},
			Err(_) => {
				println!("An error occurred, terminating connection with {}", peer_addr);
				// If remote end closed connection, then this may fail. Let it.
				let _ = stream.shutdown(Shutdown::Both);
				false
			}
		}
	}

	println!("Closed connection.");
	return Ok(());
}

fn main() {
	let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
	// accept connections and process them, spawning a new thread for each one
	println!("Server listening on port 3333");
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				println!("New connection: {}", stream.peer_addr().unwrap());
				thread::spawn(move|| {
					// connection succeeded
					handle_client(stream)
				});
			}
			Err(e) => {
				println!("Error: {}", e);
				/* connection failed */
			}
		}
	}
	// close the socket server
	drop(listener);
}

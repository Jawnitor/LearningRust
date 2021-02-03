use std::io::Read;
use std::net::TcpListener;

pub struct Server {
	s_addr: String,
}

impl Server {
	pub fn new(s_addr: String) -> Self {
		Self { s_addr }
	}

	pub fn run(self) {
		println!("listening on {}", self.s_addr);

		let listener = TcpListener::bind(&self.s_addr).unwrap();

		loop {
			// loop = while true (its infinte) //
			match listener.accept() {
				Ok((mut stream, _)) => {
					let mut buffer = [0; 1024];
					stream.read(&mut buffer);
				}
				Err(e) => println!("Failed to make connection: {}", e),
			}
		}
	}
}

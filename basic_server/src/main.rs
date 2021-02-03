mod http;
mod server;

use http::E_Method;
use http::Request;
use server::Server;

fn main() {
	let st_server: Server = Server::new("127.0.0.1:8080".to_string());
	st_server.run();
}

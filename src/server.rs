#[path = "pool.rs"]
mod pool;

use std::net::TcpListener;

pub struct ServerOptions {
    pub address: String,
    pub port: usize,
}

pub struct Server {
    target: String,
}

impl Server {
    pub fn new(options: ServerOptions) -> Server {
        Server {
            target: format!("{}:{}", options.address, options.port),
        }
    }

    pub fn listen(&mut self) {
        for stream in self.get_listener().incoming() {
            let _stream = stream.expect("Could not accept incoming connection.");
        }
    }

    fn get_listener(&mut self) -> TcpListener {
        TcpListener::bind(&self.target)
            .expect(format!("Invalid address: '{}'.", self.target).as_str())
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn correctly_format_target() {
        let server = Server::new(ServerOptions {
            address: String::from("127.0.0.1"),
            port: 3000,
        });

        assert_eq!(server.target, "127.0.0.1:3000");
    }

    #[test]
    fn bind_to_valid_address() {
        let mut server = Server::new(ServerOptions {
            address: String::from("127.0.0.1"),
            port: 3000,
        });

        server.get_listener();
    }

    #[test]
    #[should_panic(expected = "Invalid address: ':0'.")]
    fn fail_to_bind_to_invalid_address() {
        let mut server = Server::new(ServerOptions {
            address: String::from(""),
            port: 0,
        });

        server.get_listener();
    }
}

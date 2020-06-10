mod server;

use server::{Server, ServerOptions};

/// Starts the web server and listens for incoming connections.
fn main() {
    let mut server = Server::new(ServerOptions {
        address: String::from("127.0.0.1"),
        port: 3000,
    });

    server.listen();

    // let listener = get_listener("127.0.0.1:3000");
    // let pool = ThreadPool::new(4);

    // for stream in listener.incoming() {
    //     let stream = stream.expect("Could not except incoming connection.");

    //     pool.execute(|| {
    //         handle_connection(stream);
    //     });
    // }
}

// /// Returns a listener bound to the given address.
// ///
// /// ### Arguments
// ///
// /// * `address` - A valid IPv4 address with a port indicated.
// ///
// /// ### Example
// ///
// /// ```
// /// let listener = get_listener("127.0.0.1:80");
// /// ```
// fn get_listener(address: &str) -> TcpListener {
//     TcpListener::bind(address).expect(&format!("Could not bind to {}", address))
// }

// /// Handles an incoming connection.
// ///
// /// ### Arguments
// ///
// /// * `stream` - An incoming TCP stream.
// ///
// /// ### Example
// ///
// /// ```
// /// let listener = get_listener("127.0.0.1:80");
// /// let stream = listener.incoming.take(1).unwrap();
// /// handle_connection(stream);
// /// ```
// fn handle_connection(mut stream: TcpStream) {
//     let request = Request::from(&stream);
//     let response = Response::new(
//         request,
//         vec![("/", "wwwroot/hello.html"), (".*", "wwwroot/404.html")],
//     );
//     response.send(&mut stream);
// }

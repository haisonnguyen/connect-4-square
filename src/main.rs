/// Haison Nguyen
/// CS410P Rust Programming
///
/// This file acts as a manager between
/// web sockets (WS-rs) and a server (Nickel-rs)
/// The documentation for nickel can be found here:
///
/// https://nickel-org.github.io/
///
/// The documentation for ws-rs can be found here:
///
/// https://ws-rs.org/
///
/// A lot of the code in these files stem directly from
/// the basic examples on the websites, with a lot of
/// changes and custom logic added.
/// Overall architecture
/// 1. A user is able to access the game via a web interface -
/// 2. Nickel is used for routing web requests, and sends the
/// user the html
/// 3. WS web sockets are used for real time communication
/// between a client and a server
/// 4. Nickel are both separate entities
/// - Nickel is used strictly for routing and serving html
/// - WS is used strictly for real time communication
/// - They're on different ports and game is managed through
/// the web socket interface

#[macro_use]
extern crate nickel;

use ws::listen;

use connect_4_square::connect_4_square::*;
use connect_4_square::socket_server::*;

use nickel::status::StatusCode::NotFound;
use nickel::{
    Action, Continue, Halt, HttpRouter, Nickel, NickelError, Request, StaticFilesHandler,
};
use std::io::Write;

use std::thread;

fn main() {
    #[allow(unused_doc_comments)]
    /// Spawn thread to handle web routes
    thread::spawn(move || {
        println!("Spawn");
        let mut server = Nickel::new();
        server.utilize(StaticFilesHandler::new("src/assets"));

        //this is how to overwrite the default error handler to handle 404 cases with a custom view
        #[allow(clippy::extra_unused_lifetimes)]
        fn custom_404<'a>(err: &mut NickelError, _req: &mut Request) -> Action {
            if let Some(ref mut res) = err.stream {
                if res.status() == NotFound {
                    let _ = res.write_all(b"<h1>The page you're trying to access does not exsit buddy, 404 out of here!</h1>");
                    return Halt(());
                }
            }

            Continue(())
        }
        #[allow(unreachable_code)]
        server.get(
            "/",
            middleware! { |_, response|
                println!("Connection routed");
                return response.send_file("src/assets/index.html");
            },
        );

        // issue #20178
        let custom_handler: fn(&mut NickelError, &mut Request) -> Action = custom_404;

        server.handle_error(custom_handler);
        server.listen("127.0.0.1:8081").unwrap();
    });

    #[allow(unused_doc_comments)]
    /// Main thread listens to web socket
    listen("127.0.0.1:8080", |out| Server {
        out,
        game: Connect4Square::new(),
    })
    .unwrap();
}

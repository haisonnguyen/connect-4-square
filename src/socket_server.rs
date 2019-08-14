/// Haison Nguyen
/// CS410P Rust Programming
/// This file uses WS-RS to handle communication
/// between a client through the browser
/// and the game server hosted locally
use std::str;
use ws::{CloseCode, Error as wsError, Handler, Message, Result as wsResult, Sender};

use crate::connect_4_square::*;
use serde::{Deserialize, Serialize};
use serde_json::{Error as sjError, Result as sjResult};

pub struct Server {
    pub out: Sender,
    pub game: Connect4Square,
}

/// Parser parsed struct into tokens
/// passes to a macro (function) inside serder at compile time
#[derive(Serialize, Deserialize)]
pub struct CommandData {
    command: String,
}

impl Server {
    /// https://doc.rust-lang.org/std/result/
    /// Ending the expression with ? will result in the unwrapped success (Ok) value,
    /// unless the result is Err, in which case Err is returned early from the enclosing function.
    /// can only be used in functions that return Result because of the early return of Err that it provides.
    fn parse_message(&mut self, strang: String) -> Result<String, sjError> {
        // if this fails, will return Err early
        let cmd: CommandData = serde_json::from_str(&strang)?;
        println!("The value: {:?}", cmd.command);
        let command = cmd.command;

        if command.contains("Start") {
            Ok("Game Start:".to_string() + &self.game.get_current_player().to_string())
        } else if command.contains("Set Move") {
            let split: Vec<_> = command.split(':').collect();
            println!("{:?}", split[1]);
            let choice = split[1].parse::<usize>().unwrap();
            self.game.set_move(choice);
            if self.game.check_win_conditions() {
                Ok("Winner".to_string())
            } else {
                Ok("Set Move".to_string())
            }
        } else if command.contains("Reset") {
            self.game.reset_board();
            Ok("Resetted".to_string())
        } else {
            Ok("hello".to_string())
        }
    }
}

/// These handlers are called when the event
/// with a name respective to the handler name
/// is triggered
#[allow(unused_must_use)]
impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> wsResult<()> {
        if msg.is_empty() {
            self.out.send("Empty message reject");
        }
        println!("{:?}", msg);
        let text: String = msg.to_string();
        let val: sjResult<String> = self.parse_message(text);
        let val = val.unwrap();
        self.out.send(val.clone())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.")
            }
            _ => println!("The client encountered an error: {}", reason),
        }
    }

    fn on_error(&mut self, err: wsError) {
        println!("The server encounctered an error: {:?}", err);
    }
}

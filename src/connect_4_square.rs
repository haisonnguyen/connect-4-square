/// Haison Nguyen
/// CS410P Rust Programming
/// This file manages the actual game logic
/// If this was ran locally, the game would
/// automatically start by calling Connect4Square::start()
/// and take user input from console.
/// However, if ran through the web interface, then the
/// win game condition logic must be implemented on the controller
/// or in this case, Server that manages the game
/// Overall game flow:
/// 1. cplayer represents current player and makes a move,
/// 2. The game always checks if there is a winner after every move
/// 3. cplayer flip flops to represent the next player
/// 4. repeat 1-3 until win
use std::io;
extern crate rand;

#[derive(Default)]
pub struct Connect4Square {
    cplayer: u8,
    /// curent player
    board: Vec<Vec<u8>>, //our game board or vector of vectors
}

impl Connect4Square {
    /// When starting game, randomly roll a player to start
    /// Inititialize nxn matrix, add more rows and columns
    /// for longer possible games
    pub fn new() -> Connect4Square {
        Connect4Square {
            cplayer: Connect4Square::roll(),
            board: vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        }
    }

    pub fn reset_board(&mut self) {
        self.board = vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ]
    }

    pub fn is_empty(&self) -> bool {
        let mut empty = true;
        self.board.iter().for_each(|vec| {
            vec.iter().for_each(|x| {
                if *x != 0 {
                    empty = false;
                }
            })
        });

        empty
    }

    pub fn get_current_player(&self) -> u8 {
        self.cplayer
    }

    pub fn display_board(&self) {
        println!();
        for (x, y) in self.board.iter().enumerate() {
            println!("{:?}", y);
            if x == self.board.len() {
                println!("{:?} {:?}", x, y);
            }
        }
        println!();
    }

    /// This function will be used if the
    /// user wants to run this locally.
    /// Answer was modified from this example URL
    /// https://blog.v-gar.de/2019/04/rust-remove-trailing-newline-after-input/

    pub fn get_move(&self) -> usize {
        println!("Player {:?}, make your move.", self.cplayer);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let result: usize = input.trim().parse::<usize>().unwrap();
        result
    }

    /// Go through each vector (starting at last),
    /// look at the selected index, if it is 0,
    /// place the current value
    pub fn set_move(&mut self, col: usize) {
        for i in (0..self.board.len()).rev() {
            if self.board[i][col] == 0 {
                self.board[i][col] = self.cplayer;
                return;
            }

            if i == 0 && self.board[i][col] != 0 {
                println!("\nYou need to make another move\n");
            }
        }
    }

    /// Random roll for player 1/2 -> t/f
    pub fn roll() -> u8 {
        let random = rand::random();
        if random {
            1
        } else {
            2
        }
    }

    /// This is available locally
    pub fn start(&mut self) {
        let mut winner: bool = false;
        let mut choice: usize;
        while !winner {
            choice = self.get_move();
            self.set_move(choice);
            self.display_board();

            winner = self.check_win_conditions();

            if !winner {
                self.next_player();
            }
        }
        println!("Winner: Player {} !", self.cplayer);
    }

    /// Switches to the next player flip/flop
    pub fn next_player(&mut self) {
        if self.cplayer == 1 {
            self.cplayer = 2;
        } else {
            self.cplayer = 1;
        }
    }

    /// Win logic followed through tutorial
    /// https://github.com/KeithGalli/Connect4-Python/blob/master/connect4.py
    pub fn check_win_conditions(&mut self) -> bool {
        let column_count = 7;
        let row_count = 7;

        #[allow(unused_doc_comments)]
        /// Check horizontal win condition
        for i in 0..column_count - 3 {
            for j in 0..row_count {
                if self.board[j][i] == self.cplayer
                    && self.board[j][i + 1] == self.cplayer
                    && self.board[j][i + 2] == self.cplayer
                    && self.board[j][i + 3] == self.cplayer
                {
                    return true;
                }
            }
        }
        #[allow(unused_doc_comments)]
        /// Check vertical win condition
        for i in 0..column_count {
            for j in 0..row_count - 3 {
                if self.board[j][i] == self.cplayer
                    && self.board[j + 1][i] == self.cplayer
                    && self.board[j + 2][i] == self.cplayer
                    && self.board[j + 3][i] == self.cplayer
                {
                    return true;
                }
            }
        }
        #[allow(unused_doc_comments)]
        /// Check positive slope win condition
        for i in 0..column_count - 3 {
            for j in 0..row_count - 3 {
                if self.board[j][i] == self.cplayer
                    && self.board[j + 1][i + 1] == self.cplayer
                    && self.board[j + 2][i + 2] == self.cplayer
                    && self.board[j + 3][i + 3] == self.cplayer
                {
                    return true;
                }
            }
        }
        #[allow(unused_doc_comments)]
        /// Check negative slope win condition
        for i in 0..column_count - 3 {
            for j in 3..row_count {
                if self.board[j][i] == self.cplayer
                    && self.board[j - 1][i + 1] == self.cplayer
                    && self.board[j - 2][i + 2] == self.cplayer
                    && self.board[j - 3][i + 3] == self.cplayer
                {
                    return true;
                }
            }
        }
        false
    }
}

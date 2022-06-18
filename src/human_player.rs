use crate::{Board, Colour, Game, Move, Player};
use text_io::read;

pub struct HumanPlayer {
    colour: Colour,
}

impl HumanPlayer {
    fn display_board(&self, board: &Board) {
        println!(
            "\n-----------------\n{}\n-----------------\n",
            board
                .map(|row| format!("|{}|", row
                    .map(|x| match x {
                        None => " ".to_owned(),
                        Some(piece) => piece.to_string(),
                    })
                    .join("|")))
                .join("\n-----------------\n")
        )
    }
    pub fn new() -> Self {
        Self { colour: Colour::Black }
    }
}

impl Player for HumanPlayer {
    fn set_colour(&mut self, colour: Colour) -> () {
        self.colour = colour;
    }

    fn get_colour(&self) -> Colour {
        self.colour
    }

    fn get_move(&self, game: &Game) -> Move {
        match self.colour {
            Colour::Black => println!("You are playing as Black"),
            Colour::White => println!("You are playing as White"),
        }
        self.display_board(&game.board);
        loop {
            println!("Please enter your move:");
            let player_move = Move::new(read!(), read!());
            if game.is_valid_move(&player_move, self.colour) {
                return player_move;
            } else {
                println!("Invalid move, please try again");
            }
        }
    }

    fn move_made(&self, opponent_move: &Move, colour: Colour) {
        println!(
            "{} made move: {}",
            match colour {
                Colour::Black => "Black",
                Colour::White => "White",
            },
            opponent_move
        )
    }
}

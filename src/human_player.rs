use text_io::read;

use crate::{
    string_to_coords, Bishop, Board, Colour, Game, Knight, Move, Pawn, Player, Queen, Rook,
};

pub struct HumanPlayer {
    colour: Colour,
}

impl HumanPlayer {
    fn display_board(&self, board: &Board) {
        println!(
            "\n-----------------\n{}\n-----------------\n",
            board
                .map(|row| format!(
                    "|{}|",
                    row.map(|x| match x {
                        None => " ".to_owned(),
                        Some(piece) => piece.to_string(),
                    })
                    .join("|")
                ))
                .join("\n-----------------\n")
        )
    }
    pub fn new() -> Self {
        Self {
            colour: Colour::Black,
        }
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
            let mut player_move = Move::new(read!(), read!());
            let (r_src, c_src) = match string_to_coords(&player_move.src) {
                Ok((r, c)) => (r, c),
                Err(_) => {
                    println!("Invalid move, please try again");
                    continue;
                }
            };
            let (r_dst, _c_dst) = match string_to_coords(&player_move.dst) {
                Ok((r, c)) => (r, c),
                Err(_) => {
                    println!("Invalid move, please try again");
                    continue;
                }
            };
            if game.board[r_src][c_src].is_some_and(|p| p.class == Pawn(true, false))
                && (r_dst == 0 || r_dst == 7)
            {
                println!("Promotion! Please select a piece to promote to.");
                let promotion: String = read!();
                player_move = Move::promotion(
                    player_move.src,
                    player_move.dst,
                    match promotion.as_str() {
                        "Q" => Queen,
                        "B" => Bishop,
                        "N" => Knight,
                        "R" => Rook(true),
                        _ => {
                            println!("Invalid promotion, please try again");
                            continue;
                        }
                    },
                );
            }
            if game.is_valid_move(&player_move, self.colour) {
                return player_move;
            } else {
                println!("Invalid move, please try again");
                continue;
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

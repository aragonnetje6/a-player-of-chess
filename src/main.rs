extern crate core;

use std::fmt::{Display, Formatter};
use colored::Colorize;

use Colour::*;
use PieceClass::*;
use crate::human_player::HumanPlayer;

mod human_player;

fn main() {
    let player1 = HumanPlayer::new();
    let player2 = HumanPlayer::new();
    let mut game = Game::new(Box::new(player1), Box::new(player2));
    game.play();
}

type Board = [[Option<Piece>; 8]; 8];

struct Game {
    board: Board,
    players: [Box<dyn Player>; 2],
}

impl Game {
    pub fn new(mut player1: Box<dyn Player>, mut player2: Box<dyn Player>) -> Self {
        player1.set_colour(White);
        player2.set_colour(Black);
        let players = [player1, player2];
        Self {
            board: new_board(),
            players,
        }
    }

    fn play(&mut self) {
        while !self.is_finished() {
            for player in &self.players {
                let player_move = player.get_move(&self);
                if self.is_valid_move(&player_move, player.get_colour()) {
                    self.players
                        .iter()
                        .for_each(|x| x.move_made(&player_move, player.get_colour()));
                    self.execute_move(player_move);
                } else {
                    panic!("Bad move received, terminating game")
                }
            }
        }
    }
    fn is_finished(&self) -> bool {
        false
    }
    fn is_valid_move(&self, player_move: &Move, colour: Colour) -> bool {
        true
    }
    fn execute_move(&self, player_move: Move) {}
}

fn new_board() -> Board {
    let piece_classes = [
        Rook(false),
        Knight,
        Bishop,
        Queen,
        King(false),
        Bishop,
        Knight,
        Rook(false),
    ];
    let mut board = [[None; 8]; 8];
    for (i, class) in piece_classes.iter().enumerate() {
        board[0][i] = Some(Piece::new(Black, *class));
    }
    for i in 0..8 {
        board[1][i] = Some(Piece::new(Black, Pawn));
    }
    for i in 0..8 {
        board[6][i] = Some(Piece::new(White, Pawn));
    }
    for (i, class) in piece_classes.iter().enumerate() {
        board[7][i] = Some(Piece::new(White, *class));
    }
    return board;
}

#[derive(Copy, Clone)]
struct Piece {
    owner: Colour,
    class: PieceClass,
}

impl Piece {
    pub fn new(owner: Colour, class: PieceClass) -> Self {
        Self { owner, class }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.class {
                Pawn => match self.owner {
                    Black => "P".black(),
                    White => "P".white(),
                },
                Rook(_) => match self.owner {
                    Black => "R".black(),
                    White => "R".white(),
                },
                Knight => match self.owner {
                    Black => "N".black(),
                    White => "N".white(),
                },
                Bishop => match self.owner {
                    Black => "B".black(),
                    White => "B".white(),
                },
                Queen => match self.owner {
                    Black => "Q".black(),
                    White => "Q".white(),
                },
                King(_) => match self.owner {
                    Black => "K".black(),
                    White => "K".white(),
                },
            }
        )
    }
}

#[derive(Copy, Clone)]
enum PieceClass {
    Pawn,
    Rook(bool),
    Knight,
    Bishop,
    Queen,
    King(bool),
}

#[derive(Copy, Clone)]
enum Colour {
    Black,
    White,
}

trait Player {
    fn set_colour(&mut self, colour: Colour) -> ();
    fn get_colour(&self) -> Colour;
    fn get_move(&self, game: &Game) -> Move;
    fn move_made(&self, opponent_move: &Move, colour: Colour);
}

struct Move {
    src: String,
    dst: String,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {}", self.src, self.dst)
    }
}

impl Move {
    fn new(src: String, dst: String) -> Move {
        Move { src, dst }
    }
}

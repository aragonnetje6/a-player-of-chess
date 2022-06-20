#![feature(is_some_with)]
extern crate core;

use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use colored::Colorize;

use Colour::*;
use PieceClass::*;

use crate::human_player::HumanPlayer;

mod human_player;

fn main() {
    let player1 = HumanPlayer::new();
    let player2 = HumanPlayer::new();
    let mut game = Game::new(Box::new(player1), Box::new(player2));
    let vec = game.get_available_moves(White);
    println!("{:?}", vec);
    println!("{}", vec.len());
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
                    self.execute_move(&player_move);
                    self.players
                        .iter()
                        .for_each(|x| x.move_made(&player_move, player.get_colour()));
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
        self.get_available_moves(colour).contains(player_move)
    }
    fn execute_move(&self, _player_move: &Move) {
        todo!();
    }

    fn get_available_moves(&self, colour: Colour) -> Vec<Move> {
        let mut moves = vec![];
        for (r, row) in self.board.iter().enumerate() {
            for (c, piece_opt) in row.iter().enumerate() {
                match piece_opt {
                    None => {}
                    Some(piece) => {
                        if piece.owner != colour {
                            continue;
                        }
                        match piece.class {
                            Pawn(moved, _) => self.get_pawn_moves(colour, &mut moves, r, c, moved),
                            Rook(_) => self.get_rook_moves(colour, &mut moves, r, c),
                            Knight => self.get_knight_moves(colour, &mut moves, r, c),
                            Bishop => self.get_bishop_moves(colour, &mut moves, r, c),
                            Queen => self.get_queen_moves(colour, &mut moves, r, c),
                            King(moved) => self.get_king_moves(colour, &mut moves, r, c, moved),
                        }
                    }
                }
            }
        }
        // todo: filter moves that result in check
        return moves;
    }

    fn get_linear_moves(
        &self,
        colour: Colour,
        moves: &mut Vec<Move>,
        r: usize,
        c: usize,
        r_dir: isize,
        c_dir: isize,
    ) {
        for i in 1..8 {
            let terminated = self.get_stepwise_move(colour, moves, r, c, i * r_dir, i * c_dir);
            if terminated {
                break;
            }
        }
    }

    fn get_stepwise_move(
        &self,
        colour: Colour,
        moves: &mut Vec<Move>,
        r: usize,
        c: usize,
        r_dir: isize,
        c_dir: isize,
    ) -> bool {
        let new_r: isize = r as isize + r_dir;
        let new_c: isize = c as isize + c_dir;
        if new_r < 0 || new_r >= 8 || new_c < 0 || new_c >= 8 {
            return false;
        }
        match self.board[new_r as usize][new_c as usize] {
            None => moves.push(Move::new(
                coords_to_string(r, c),
                coords_to_string(new_r as usize, new_c as usize),
            )),
            Some(other) => {
                if other.owner != colour {
                    moves.push(Move::new(
                        coords_to_string(r, c),
                        coords_to_string(new_r as usize, new_c as usize),
                    ))
                }
                return true;
            }
        }
        return false;
    }

    fn get_king_moves(
        &self,
        colour: Colour,
        moves: &mut Vec<Move>,
        r: usize,
        c: usize,
        moved: bool,
    ) {
        // main moves
        for (r_dir, c_dir) in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
        ] {
            self.get_stepwise_move(colour, moves, r, c, r_dir, c_dir);
        }
        // Castle left
        if !moved
            && self.board[r][0].is_some_and(|p| *p == Piece::new(colour, Rook(true)))
            && (1..c).all(|i| self.board[r][i].is_none())
        {
            moves.push(Move::new(
                coords_to_string(r, c),
                coords_to_string(r, c - 2),
            ))
        }
        // Castle right
        if !moved
            && self.board[r][7].is_some_and(|p| *p == Piece::new(colour, Rook(true)))
            && (c + 1..7).all(|i| self.board[r][i].is_none())
        {
            moves.push(Move::new(
                coords_to_string(r, c),
                coords_to_string(r, c + 2),
            ))
        }
    }

    fn get_knight_moves(&self, colour: Colour, moves: &mut Vec<Move>, r: usize, c: usize) {
        for (r_dir, c_dir) in [
            (-2, -1),
            (-2, 1),
            (2, -1),
            (2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
        ] {
            self.get_stepwise_move(colour, moves, r, c, r_dir, c_dir);
        }
    }
    fn get_queen_moves(&self, colour: Colour, moves: &mut Vec<Move>, r: usize, c: usize) {
        for (r_dir, c_dir) in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
        ] {
            self.get_linear_moves(colour, moves, r, c, r_dir, c_dir);
        }
    }

    fn get_bishop_moves(&self, colour: Colour, moves: &mut Vec<Move>, r: usize, c: usize) {
        for (r_dir, c_dir) in [(1, 1), (-1, -1), (-1, 1), (1, -1)] {
            self.get_linear_moves(colour, moves, r, c, r_dir, c_dir);
        }
    }

    fn get_rook_moves(&self, colour: Colour, moves: &mut Vec<Move>, r: usize, c: usize) {
        for (r_dir, c_dir) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            self.get_linear_moves(colour, moves, r, c, r_dir, c_dir);
        }
    }

    fn get_pawn_moves(
        &self,
        colour: Colour,
        moves: &mut Vec<Move>,
        r: usize,
        c: usize,
        moved: bool,
    ) {
        let dir: isize = match colour {
            Black => 1,
            White => -1,
        };
        let new_r = r as isize + dir;
        let new_r_2 = r as isize + dir * 2;
        if new_r < 0 || new_r >= 8 {
            return;
        }
        if self.board[new_r as usize][c].is_none() {
            if new_r == 0 && new_r == 7 {
                for class in [Queen, Knight, Rook(true), Bishop] {
                    moves.push(Move::promotion(
                        coords_to_string(r, c),
                        coords_to_string(new_r as usize, c),
                        class,
                    ))
                }
            } else {
                moves.push(Move::new(
                    coords_to_string(r, c),
                    coords_to_string(new_r as usize, c),
                ));
            }
        }
        if !moved && self.board[new_r_2 as usize][c].is_none() {
            moves.push(Move::new(
                coords_to_string(r, c),
                coords_to_string(new_r_2 as usize, c),
            ));
        }
        if c + 1 < 8 && self.board[new_r as usize][c + 1].is_some_and(|other| other.owner != colour)
        {
            if new_r == 0 && new_r == 7 {
                for class in [Queen, Knight, Rook(true), Bishop] {
                    moves.push(Move::promotion(
                        coords_to_string(r, c),
                        coords_to_string(new_r as usize, c + 1),
                        class,
                    ))
                }
            } else {
                moves.push(Move::new(
                    coords_to_string(r, c),
                    coords_to_string(new_r as usize, c + 1),
                ));
            }
        }
        if c.checked_sub(1).is_some()
            && self.board[new_r as usize][c - 1].is_some_and(|other| other.owner != colour)
        {
            if new_r == 0 && new_r == 7 {
                for class in [Queen, Knight, Rook(true), Bishop] {
                    moves.push(Move::promotion(
                        coords_to_string(r, c),
                        coords_to_string(new_r as usize, c - 1),
                        class,
                    ))
                }
            } else {
                moves.push(Move::new(
                    coords_to_string(r, c),
                    coords_to_string(new_r as usize, c - 1),
                ));
            }
        }
        if c.checked_sub(1).is_some()
            && self.board[r][c - 1]
                .is_some_and(|p| p.owner != colour && p.class == Pawn(true, true))
        {
            moves.push(Move::new(
                coords_to_string(r, c),
                coords_to_string(new_r as usize, c - 1),
            ));
        }
        if c + 1 < 8
            && self.board[r][c + 1]
                .is_some_and(|p| p.owner != colour && p.class == Pawn(true, true))
        {
            moves.push(Move::new(
                coords_to_string(r, c),
                coords_to_string(new_r as usize, c + 1),
            ));
        }
    }
}

fn coords_to_string(row: usize, col: usize) -> String {
    format!(
        "{}{}",
        match col {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => panic!("Invalid coordinate"),
        },
        row.to_string()
    )
}

fn string_to_coords(square: &str) -> Result<(usize, usize), &str> {
    let (c, r) = square.split_at(1);
    Ok((
        match r.parse::<usize>() {
            Ok(x) => x,
            Err(_) => {
                return Err("Invalid row");
            }
        },
        match c {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => {
                return Err("Invalid column");
            }
        },
    ))
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
        board[1][i] = Some(Piece::new(Black, Pawn(false, false)));
    }
    for i in 0..8 {
        board[6][i] = Some(Piece::new(White, Pawn(false, false)));
    }
    for (i, class) in piece_classes.iter().enumerate() {
        board[7][i] = Some(Piece::new(White, *class));
    }
    return board;
}

#[derive(Copy, Clone, PartialEq)]
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
                Pawn(_, _) => match self.owner {
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

#[derive(Copy, Clone, PartialEq, Debug)]
enum PieceClass {
    Pawn(bool, bool),
    Rook(bool),
    Knight,
    Bishop,
    Queen,
    King(bool),
}

#[derive(Copy, Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
struct Move {
    src: String,
    dst: String,
    promotion: Option<PieceClass>,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {}", self.src, self.dst)
    }
}

impl Move {
    fn new(src: String, dst: String) -> Move {
        Self {
            src: src.to_lowercase(),
            dst: dst.to_lowercase(),
            promotion: None,
        }
    }
    fn promotion(src: String, dst: String, promote_to: PieceClass) -> Move {
        Self {
            src: src.to_lowercase(),
            dst: dst.to_lowercase(),
            promotion: Some(promote_to),
        }
    }
}

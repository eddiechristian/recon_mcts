use std::{collections::HashMap, convert::From};

use crate::chess::{chess_mcts::Player, knight::get_knight_unvalidated_moves};

use super::{
    chess_errors, chess_notation, fen::FenRecord, pawn::get_pawn_unvalidated_moves,
    rook::get_rook_unvalidated_moves,
};

pub(crate) const INIT: Option<Piece> = None;

pub const FEN_INITIAL_STATE: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// returns UCI long algebraic notation
/// examples:  e2e4, e7e5, e1g1 (for white short castling), e7e8q (for promotion)
pub fn get_avaiable_actions(state: &str) -> Vec<[char; 5]> {
    let a = Vec::new();

    a
}
pub fn get_unvalidated_moves(
    state: &str,
) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
    let chess = Chess::from(&FenRecord::from(&state.to_owned()));
    return chess.get_unvalidated_moves();

}




#[derive(Debug, PartialEq)]
pub(crate) enum PieceType {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}
#[derive(Debug)]
pub(crate) struct Piece {
    pub piece_type: PieceType,
    pub moved: bool,
}

impl Piece {
    pub fn get_player(&self) -> Player {
        match self.piece_type {
            PieceType::WhitePawn
            | PieceType::WhiteRook
            | PieceType::WhiteKnight
            | PieceType::WhiteBishop
            | PieceType::WhiteQueen
            | PieceType::WhiteKing => {
                return Player::White;
            }
            _ => {
                return Player::Black;
            }
        }
    }

    fn get_unvalidated_moves(
        &self,
        state: &[Option<Piece>; 64],
        spot: &str,
    ) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
        match self.piece_type {
            PieceType::WhitePawn => {
                return get_pawn_unvalidated_moves(&self, state, spot);
            }
            _ => {
                todo!();
            }
        }
    }
}

pub(crate) struct Chess {
    pub state: [Option<Piece>; 64],
    pub player: Player,
    pub castling: String,
    pub halfmove_clock: u8,
    pub full_move_number: u16,
}

impl Chess {
pub fn get_unvalidated_moves(&self
) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
    let mut unvalidated_moves = HashMap::new();
    for (index, piece_opt) in self.state.iter().enumerate() {
        if let Some(piece) = piece_opt {
            match piece.piece_type {
                PieceType::WhitePawn | PieceType::BlackPawn => {
                    let mut unvalidated_moves_pawn = get_pawn_unvalidated_moves(
                        piece,
                        &self.state,
                        &chess_notation::index_to_spot(index),
                    )?;
                    for (key, mut value) in unvalidated_moves_pawn.iter_mut() {
                        unvalidated_moves
                            .entry(key.to_owned())
                            .or_insert_with(|| Vec::new())
                            .append(&mut value);
                    }
                },
                PieceType::WhiteRook | PieceType::BlackRook => {
                    let mut unvalidated_moves_rook = get_rook_unvalidated_moves(
                        piece,
                        &self.state,
                        &chess_notation::index_to_spot(index),
                    )?;
                    for (key, mut value) in unvalidated_moves_rook.iter_mut() {
                        unvalidated_moves
                            .entry(key.to_owned())
                            .or_insert_with(|| Vec::new())
                            .append(&mut value);
                    }
                },
                PieceType::WhiteKnight | PieceType::BlackKnight => {
                    let mut unvalidated_moves_rook = get_knight_unvalidated_moves(
                        piece,
                        &self.state,
                        &chess_notation::index_to_spot(index),
                    )?;
                    for (key, mut value) in unvalidated_moves_rook.iter_mut() {
                        unvalidated_moves
                            .entry(key.to_owned())
                            .or_insert_with(|| Vec::new())
                            .append(&mut value);
                    }
                },
                PieceType::WhiteBishop | PieceType::BlackBishop => {

                }
                _ => {}
            }
        }
    }
    println!("unvalidated_moves {:?}", unvalidated_moves);
    Ok(unvalidated_moves)
}
}

use std::{collections::HashMap, convert::From, fmt::format};

use crate::chess::{chess_mcts::Player, king::get_king_unvalidated_moves};

use super::{
    chess_errors, chess_notation, fen::FenRecord, knight::get_knight_unvalidated_moves,
    pawn::get_pawn_unvalidated_moves, queen::get_queen_unvalidated_moves,
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

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

pub enum MoveType {
    Enpassant(usize),
    Castling,
    Regular,
    Promotion(char)
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
    fn move_horizontal(&self, to_spot: &str, state: &[Option<Piece>; 64], delta_x: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        match self.piece_type {
            PieceType::BlackPawn | PieceType::WhitePawn | 
            PieceType::BlackBishop | PieceType::WhiteBishop |
            PieceType::BlackKnight | PieceType::WhiteKnight => {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidMove(msg));
            },
            PieceType::BlackQueen | PieceType::WhiteQueen |
            PieceType::BlackRook | PieceType::WhiteRook
            => {
                if promotion.is_some() {
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
                }
                Ok(to_spot.to_string())
            }
            PieceType::BlackKing | PieceType::WhiteKing => {
                todo!()
            }

        }
    }
    fn move_vertical(&self, to_spot: &str, state: &[Option<Piece>; 64], delta_y: i8, promotion: Option<&str>) -> Result<(String,MoveType), chess_errors::ChessErrors>{
        match self.piece_type {
           PieceType::BlackBishop | PieceType::WhiteBishop |
            PieceType::BlackKnight | PieceType::WhiteKnight => {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidMove(msg));
            },
            PieceType::BlackQueen | PieceType::WhiteQueen |
            PieceType::BlackRook | PieceType::WhiteRook |
            PieceType::BlackKing | PieceType::WhiteKing
            => {
                if promotion.is_some() {
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
                }
                Ok((to_spot.to_string(),MoveType::Regular))
            }
            PieceType::BlackPawn | PieceType::WhitePawn => {
                todo!()
            }

        }
    }
    fn move_diagonal(&self, to_spot: &str, state: &[Option<Piece>; 64], delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        match self.piece_type {
            PieceType::BlackKnight | PieceType::WhiteKnight |
            PieceType::BlackRook | PieceType::WhiteRook => {
                 let msg = format!("{}",to_spot);
                 return Err(chess_errors::ChessErrors::InvalidMove(msg));
             },
             PieceType::BlackQueen | PieceType::WhiteQueen |
             PieceType::BlackKing | PieceType::WhiteKing | 
             PieceType::BlackBishop | PieceType::WhiteBishop
             => {
                 if promotion.is_some() {
                     let msg = format!("{}",to_spot);
                     return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
                 }
                 Ok(to_spot.to_string())
             }
             PieceType::BlackPawn | PieceType::WhitePawn => {
                 todo!()
             }
 
         }
    }
    fn move_knight(&self, to_spot: &str, state: &[Option<Piece>; 64], promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        match self.piece_type {
            PieceType::BlackRook | PieceType::WhiteRook |
            PieceType::BlackQueen | PieceType::WhiteQueen |
            PieceType::BlackKing | PieceType::WhiteKing | 
            PieceType::BlackBishop | PieceType::WhiteBishop |
            PieceType::BlackPawn | PieceType::WhitePawn => {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidMove(msg));
            }
            PieceType::BlackKnight | PieceType::WhiteKnight => {
                todo!()
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
    pub en_passant_enabled: Option<String>,
}

impl Chess {
    pub fn get_unvalidated_moves(
        &self,
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
                        //check for promotion
                        let row = index/8;
                        if row == 6 &&  piece.piece_type == PieceType::WhitePawn{
                            todo!("Add diagonal");
                            let spot = chess_notation::index_to_spot(index);
                            let to_spot_queen = format!("{}8q",spot.chars().nth(0).unwrap());
                            let to_spot_rook = format!("{}8r",spot.chars().nth(0).unwrap());
                            let to_spot_knight = format!("{}8n",spot.chars().nth(0).unwrap());
                            let to_spot_bishop = format!("{}8b",spot.chars().nth(0).unwrap());
                            unvalidated_moves.insert(spot.clone(), Vec::new());
                            unvalidated_moves
                                .entry(spot.to_owned())
                                .or_insert_with(|| Vec::new())
                                .push(to_spot_queen);
                            unvalidated_moves
                            .entry(spot.to_owned())
                            .or_insert_with(|| Vec::new())
                            .push(to_spot_rook);
                            unvalidated_moves
                            .entry(spot.to_owned())
                            .or_insert_with(|| Vec::new())
                            .push(to_spot_knight);
                            unvalidated_moves
                            .entry(spot.to_owned())
                            .or_insert_with(|| Vec::new())
                            .push(to_spot_bishop);
                        } else if row == 1 &&  piece.piece_type == PieceType::BlackPawn{
                            todo!("Add diagonal");
                            let spot = chess_notation::index_to_spot(index);
                            let to_spot_queen = format!("{}1q",spot.chars().nth(0).unwrap());
                            let to_spot_rook = format!("{}1r",spot.chars().nth(0).unwrap());
                            let to_spot_knight = format!("{}1n",spot.chars().nth(0).unwrap());
                            let to_spot_bishop = format!("{}1b",spot.chars().nth(0).unwrap());
                            unvalidated_moves.insert(spot.clone(), Vec::new());

                            unvalidated_moves
                                .entry(spot.to_owned())
                                .or_insert_with(|| Vec::new())
                                .push(to_spot_queen);
                            unvalidated_moves
                            .entry(spot.to_owned())
                            .or_insert_with(|| Vec::new())
                            .push(to_spot_rook);
                            unvalidated_moves
                            .entry(spot.to_owned())
                            .or_insert_with(|| Vec::new())
                            .push(to_spot_knight);
                            unvalidated_moves
                            .entry(spot.to_owned())
                            .or_insert_with(|| Vec::new())
                            .push(to_spot_bishop);
                            println!("pawn unvalidated {:?}",unvalidated_moves );
                        }
                    }
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
                    }
                    PieceType::WhiteKnight | PieceType::BlackKnight => {
                        let mut unvalidated_moves_knight = get_knight_unvalidated_moves(
                            piece,
                            &self.state,
                            &chess_notation::index_to_spot(index),
                        )?;
                        for (key, mut value) in unvalidated_moves_knight.iter_mut() {
                            unvalidated_moves
                                .entry(key.to_owned())
                                .or_insert_with(|| Vec::new())
                                .append(&mut value);
                        }
                    }
                    PieceType::WhiteBishop | PieceType::BlackBishop => {
                        let mut unvalidated_moves_bishop = get_rook_unvalidated_moves(
                            piece,
                            &self.state,
                            &chess_notation::index_to_spot(index),
                        )?;
                        for (key, mut value) in unvalidated_moves_bishop.iter_mut() {
                            unvalidated_moves
                                .entry(key.to_owned())
                                .or_insert_with(|| Vec::new())
                                .append(&mut value);
                        }
                    }
                    PieceType::WhiteQueen | PieceType::BlackQueen => {
                        let mut unvalidated_moves_queen = get_queen_unvalidated_moves(
                            piece,
                            &self.state,
                            &chess_notation::index_to_spot(index),
                        )?;
                        for (key, mut value) in unvalidated_moves_queen.iter_mut() {
                            unvalidated_moves
                                .entry(key.to_owned())
                                .or_insert_with(|| Vec::new())
                                .append(&mut value);
                        }
                    }
                    PieceType::WhiteKing | PieceType::BlackKing => {
                        let mut unvalidated_moves_king = get_king_unvalidated_moves(
                            piece,
                            &self.state,
                            &chess_notation::index_to_spot(index),
                        )?;
                        for (key, mut value) in unvalidated_moves_king.iter_mut() {
                            unvalidated_moves
                                .entry(key.to_owned())
                                .or_insert_with(|| Vec::new())
                                .append(&mut value);
                        }
                    }
                    _ => {}
                }
            }
        }
       
        Ok(unvalidated_moves)
    }

    pub fn is_move_valid(&self, from_spot: &str, to_spot: &str, whos_turn: Player, promotion_opt: Option<&str>)->Result<(MoveType), chess_errors::ChessErrors> {
        // first determine if piece at from is correct player.
        if let Ok(index) = chess_notation::notation_to_index(&from_spot) {
            if let Some(piece) = &self.state[index] {
                if piece.get_player() != whos_turn{
                    let msg = format!("{}",from_spot);
                    return Err(chess_errors::ChessErrors::WrongPlayer(msg));
                }
            } else {
                let msg = format!("{}",from_spot);
                return Err(chess_errors::ChessErrors::NoPiece(msg));
            }
        }
        //if too spot is current player its invalid
        if let Ok(index) = chess_notation::notation_to_index(&to_spot) {
            if let Some(piece) = &self.state[index] {
                if piece.get_player() == whos_turn{
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::PlayerPieceAlreadyThere(msg));
                }
            } 
        }
        if  promotion_opt.is_some() {
            //promotions are only valid from 8th rank for pawn
            let from_row = chess_notation::convert_row(from_spot)?;
            let to_row = chess_notation::convert_row(to_spot)?;
            if whos_turn == Player::White && to_row !=  0 &&  from_row != 1 {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
            }
            if whos_turn == Player::Black && to_row != 8 && from_row!= 7 {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
            }
        }

        // the x and y deltas will tell what kind of move it is
        
        let (from_point, to_point) = chess_notation::convert_move_notation_to_xy(from_spot,to_spot)?;
        let delta_x: i8 = (from_point.x as i8 - to_point.x as i8) as i8;
        let delta_y: i8 = (from_point.y as i8 - to_point.y as i8) as i8;
        if delta_x == 0  {
            //vertical
            let dir = {
                if delta_y < 0 {
                    Direction::Down
                }else  {
                    Direction::Up
                }
            }; 
            if delta_y.abs() !=1 {
                self.is_path_blocked(from_spot, to_spot, dir)?;
            }
            if let Ok(index) = chess_notation::notation_to_index(&from_spot) {
                if let Some(piece) = &self.state[index] {
                    if let (_, MoveType::Promotion(new_piece)) = piece.move_vertical(to_spot, &self.state, delta_y, promotion_opt)?{
                        return Ok(MoveType::Promotion(new_piece));
                    }
                }
            }
        } else if delta_y == 0{
            //Horiz
            let dir = {
                if delta_x < 0 {
                    Direction::Right
                }else  {
                    Direction::Left
                }
            }; 
            if delta_x.abs() !=1 {
                self.is_path_blocked(from_spot, to_spot, dir)?;
            }
            if let Ok(index) = chess_notation::notation_to_index(&from_spot) {
                if let Some(piece) = &self.state[index] {
                    piece.move_horizontal(to_spot, &self.state, delta_x, promotion_opt)?;
                }
            }
        }else if delta_x.abs() == delta_y.abs(){
            //diagonal
            //determine dir
            let dir = {
                if delta_x > 0 && delta_y > 0  {
                    Direction::UpLeft
                } else if delta_x > 0 && delta_y < 0 {
                    Direction::DownLeft
                } else if delta_x < 0 && delta_y < 0 {
                    Direction::DownRight
                } else {
                    Direction::UpRight
                }
            };
            if delta_x.abs() != 1 {
                //check pieces between because multiple spaces
                self.is_path_blocked(from_spot, to_spot, dir)?;
            }
            if let Ok(index) = chess_notation::notation_to_index(&from_spot) {
                if let Some(piece) = &self.state[index] {
                    piece.move_diagonal(to_spot, &self.state, delta_y, promotion_opt)?;
                }
            }
            // if diagonal deltas must be equal, except for Knight

        }else if (delta_x.abs() == 2 && delta_y.abs() ==1) || (delta_x.abs() == 1 && delta_y.abs() ==2){
            if let Ok(index) = chess_notation::notation_to_index(&from_spot) {
                if let Some(piece) = &self.state[index] {
                    piece.move_knight(to_spot, &self.state, promotion_opt)?;
                }
            }
        }

        //check for current player in check
        Ok(MoveType::Regular)
    }

    pub fn is_path_blocked(&self, from_spot: &str, to_spot: &str, dir: Direction)-> Result<(), chess_errors::ChessErrors>{
        let mut pos:String = to_spot.to_string();
        loop{
            if let Ok(bounds) = chess_notation::get_bounds(&pos){
                let next_pos_opt=  match dir{
                    Direction::Up => {
                            bounds.bottom
                    },
                    Direction::Down => {
                            bounds.top
                    },
                    Direction::Left => {
                            bounds.right
                    },
                    Direction::Right => {
                            bounds.left
                    },
                    Direction::DownLeft => {
                            bounds.top_right_diag
                    },
                    Direction::UpLeft => {
                            bounds.bottom_right_diag
                    },
                    Direction::UpRight => {
                            bounds.bottom_left_diag
                    },
                    Direction::DownRight => {
                            bounds.top_left_diag
                    },
                };
                if let Some(next_pos_array) = next_pos_opt {
                    let x = std::str::from_utf8(&next_pos_array).unwrap();
                    pos= x.to_string();
                    if pos == from_spot {
                        break;
                    }
                    if let Ok(index) = chess_notation::notation_to_index(&pos) {
                        if let Some(piece) = &self.state[index] {
                            if piece.get_player() == self.player{
                                return Err(chess_errors::ChessErrors::PieceBetween(pos));
                            }
                        }
                    }
                }else {
                    println!("oops1");
                    return Err(chess_errors::ChessErrors::InvalidNotation(pos.to_string()));
                }
            } else {

                println!("oops2");
                return Err(chess_errors::ChessErrors::InvalidNotation(pos.to_string()));
            }
           
        }
        Ok(())
    }
    
}

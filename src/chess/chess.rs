use core::num;
use std::{collections::HashMap, convert::From, fmt::format};

use crate::chess::{chess_mcts::Player, king::get_king_unvalidated_moves};

use super::{
    chess_errors, chess_notation, fen::FenRecord, knight::get_knight_unvalidated_moves,
    pawn::get_pawn_unvalidated_moves, pawn::move_pawn_vertical , pawn::move_pawn_diagonal,
    queen::get_queen_unvalidated_moves, rook::get_rook_unvalidated_moves,
};

pub(crate) const INIT: Option<Piece> = None;

pub const FEN_INITIAL_STATE: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// returns UCI long algebraic notation
/// examples:  e2e4, e7e5, e1g1 (for white short castling), e7e8q (for promotion)
pub fn get_avaiable_actions(state: &str) -> Vec<[char; 5]> {
    let a = Vec::new();

    a
}
pub fn get_legal_moves(
    state: &str,
) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
    let chess = Chess::from(&FenRecord::from(&state.to_owned()));
    let mut x: FenRecord = FenRecord::from(&chess);
    println!("{}\n{}",state, x.to_string());
    chess.get_legal_moves()
    
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

pub(crate) enum MoveType {
    Enpassant(usize),
    Castling,
    Regular,
    Promotion(PieceType)
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
                move_pawn_vertical(self, to_spot, state, delta_y, promotion)
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
                move_pawn_diagonal(self, to_spot, state, delta_y, promotion)
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
                Ok(to_spot.to_string())
            }
    }
}
}

pub(crate) struct Chess {
    pub state: [Option<Piece>; 64],
    pub player: Player,
    pub castling: Option<String>,
    pub halfmove_clock: u8,
    pub full_move_number: u16,
    pub en_passant_target: Option<String>,
}

impl From<&Chess> for FenRecord {
    fn from(chess: &Chess) -> Self {
        let mut piece_placement_data = "".to_owned();
        let mut num_empty = 0;
        for (index, piece_opt) in chess.state.iter().enumerate() {
            if index != 0 && index % 8 == 0 {
               if num_empty > 0 {
                    piece_placement_data.push_str(&format!("{}", num_empty));
                }
                piece_placement_data.push_str("/");
                num_empty = 0;
            }
            if let Some(piece) = piece_opt {
                if num_empty > 0 {
                    piece_placement_data.push_str(&format!("{}", num_empty));
                    num_empty = 0;
                } else {
                    let piece_type =
                    match piece.piece_type {
                        PieceType::BlackBishop => "b",
                        PieceType::BlackKing =>   "k",
                        PieceType::BlackKnight => "n",
                        PieceType::BlackPawn =>   "p",
                        PieceType::BlackQueen =>  "q",
                        PieceType::BlackRook =>   "r",
                        PieceType::WhiteBishop => "B",
                        PieceType::WhiteKing =>   "K",
                        PieceType::WhiteKnight => "N",
                        PieceType::WhitePawn =>   "P",
                        PieceType::WhiteQueen =>  "Q",
                        PieceType::WhiteRook =>   "R",

                    };
                    piece_placement_data.push_str(piece_type);
                }
            } else {
                num_empty+= 1;
            }
        }
        let player = match chess.player {
            Player::Black => 'b',
            Player::White => 'w'
        };
        let en_passant_target = match &chess.en_passant_target {
            Some(target) => target.clone(),
            None => "-".to_owned()
        };
        let castling = match &chess.castling {
            Some(castling) => castling.clone(),
            None => "-".to_owned()
        };
        FenRecord {
            piece_placement_data: piece_placement_data,
            player: player,
            castling: castling,
            en_passant_target: en_passant_target,
            halfmove_clock: chess.halfmove_clock,
            full_move_number: chess.full_move_number,
        }
    }
}

impl Chess {

    pub fn get_legal_moves(&self) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors>{
        
        let mut legal_moves_map = HashMap::new();
        let mut unvalidated_moves = self.get_unvalidated_moves();
       for (from_spot, to_spots) in unvalidated_moves.unwrap() {
            let mut legal_moves_vec = Vec::new();
            for to_spot in to_spots {
                if self.is_move_valid(&from_spot, &to_spot , None).is_ok(){

                    legal_moves_vec.push(format!("{}{}",from_spot, to_spot));
                }
            }
            legal_moves_map.insert(from_spot, legal_moves_vec);
       }
        //println!("validated_moves: {:?}", legal_moves_map);
        Ok(legal_moves_map)
    }

    pub fn get_unvalidated_moves(
        &self,
    ) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
    
        let mut unvalidated_moves = HashMap::new();
        for (index, piece_opt) in self.state.iter().enumerate() {
            if let Some(piece) = piece_opt {
                if piece.get_player() == self.player {
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
                        let col = index % 8;
                        if row == 1 &&  piece.piece_type == PieceType::WhitePawn{
                            let mut unvalidated_moves_vec = Vec::new();
                            let spot = chess_notation::index_to_spot(index);

                            let to_spot_queen = format!("{}8q",spot.chars().nth(0).unwrap());
                            let to_spot_rook = format!("{}8r",spot.chars().nth(0).unwrap());
                            let to_spot_knight = format!("{}8n",spot.chars().nth(0).unwrap());
                            let to_spot_bishop = format!("{}8b",spot.chars().nth(0).unwrap());
                            unvalidated_moves_vec.push(to_spot_queen);
                            unvalidated_moves_vec.push(to_spot_rook);
                            unvalidated_moves_vec.push(to_spot_knight);
                            unvalidated_moves_vec.push(to_spot_bishop);
                            
                            if col ==0 {
                                let right_spot= chess_notation::index_to_spot(index+1); 
                                let right_to_spot_queen = format!("{}8q",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_rook = format!("{}8r",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_knight = format!("{}8n",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_bishop = format!("{}8b",right_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(right_to_spot_queen);
                                unvalidated_moves_vec.push(right_to_spot_rook);
                                unvalidated_moves_vec.push(right_to_spot_knight);
                                unvalidated_moves_vec.push(right_to_spot_bishop);
                            }else if col == 7 {
                                let left_spot= chess_notation::index_to_spot(index-1); 
                                let left_to_spot_queen = format!("{}8q",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_rook = format!("{}8r",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_knight = format!("{}8n",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_bishop = format!("{}8b",left_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(left_to_spot_queen);
                                unvalidated_moves_vec.push(left_to_spot_rook);
                                unvalidated_moves_vec.push(left_to_spot_knight);
                                unvalidated_moves_vec.push(left_to_spot_bishop);
                            }else {
                                let right_spot= chess_notation::index_to_spot(index+1); 
                                let right_to_spot_queen = format!("{}8q",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_rook = format!("{}8r",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_knight = format!("{}8n",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_bishop = format!("{}8b",right_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(right_to_spot_queen);
                                unvalidated_moves_vec.push(right_to_spot_rook);
                                unvalidated_moves_vec.push(right_to_spot_knight);
                                unvalidated_moves_vec.push(right_to_spot_bishop);
                                let left_spot= chess_notation::index_to_spot(index-1); 
                                let left_to_spot_queen = format!("{}8q",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_rook = format!("{}8r",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_knight = format!("{}8n",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_bishop = format!("{}8b",left_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(left_to_spot_queen);
                                unvalidated_moves_vec.push(left_to_spot_rook);
                                unvalidated_moves_vec.push(left_to_spot_knight);
                                unvalidated_moves_vec.push(left_to_spot_bishop);
                            }
                            unvalidated_moves
                            .entry(spot)
                            .or_insert_with(|| Vec::new())
                            .append(&mut unvalidated_moves_vec);
                        } else if row == 6 &&  piece.piece_type == PieceType::BlackPawn{
                            let mut unvalidated_moves_vec = Vec::new();

                            let spot = chess_notation::index_to_spot(index);
                            let to_spot_queen = format!("{}1q",spot.chars().nth(0).unwrap());
                            let to_spot_rook = format!("{}1r",spot.chars().nth(0).unwrap());
                            let to_spot_knight = format!("{}1n",spot.chars().nth(0).unwrap());
                            let to_spot_bishop = format!("{}1b",spot.chars().nth(0).unwrap());
                            unvalidated_moves_vec.push(to_spot_queen);
                            unvalidated_moves_vec.push(to_spot_rook);
                            unvalidated_moves_vec.push(to_spot_knight);
                            unvalidated_moves_vec.push(to_spot_bishop);
                            if col ==0 {
                                let right_spot= chess_notation::index_to_spot(index+1); 
                                let right_to_spot_queen = format!("{}1q",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_rook = format!("{}1r",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_knight = format!("{}1n",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_bishop = format!("{}1b",right_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(right_to_spot_queen);
                                unvalidated_moves_vec.push(right_to_spot_rook);
                                unvalidated_moves_vec.push(right_to_spot_knight);
                                unvalidated_moves_vec.push(right_to_spot_bishop);
                            }else if col == 7 {
                                let left_spot= chess_notation::index_to_spot(index-1); 
                                let left_to_spot_queen = format!("{}1q",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_rook = format!("{}1r",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_knight = format!("{}1n",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_bishop = format!("{}1b",left_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(left_to_spot_queen);
                                unvalidated_moves_vec.push(left_to_spot_rook);
                                unvalidated_moves_vec.push(left_to_spot_knight);
                                unvalidated_moves_vec.push(left_to_spot_bishop);
                            } else {
                                let right_spot= chess_notation::index_to_spot(index+1); 
                                let right_to_spot_queen = format!("{}1q",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_rook = format!("{}1r",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_knight = format!("{}1n",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_bishop = format!("{}1b",right_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(right_to_spot_queen);
                                unvalidated_moves_vec.push(right_to_spot_rook);
                                unvalidated_moves_vec.push(right_to_spot_knight);
                                unvalidated_moves_vec.push(right_to_spot_bishop);
                                let left_spot= chess_notation::index_to_spot(index-1); 
                                let left_to_spot_queen = format!("{}1q",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_rook = format!("{}1r",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_knight = format!("{}1n",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_bishop = format!("{}1b",left_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(left_to_spot_queen);
                                unvalidated_moves_vec.push(left_to_spot_rook);
                                unvalidated_moves_vec.push(left_to_spot_knight);
                                unvalidated_moves_vec.push(left_to_spot_bishop);
                            }
                            unvalidated_moves
                            .entry(spot)
                            .or_insert_with(|| Vec::new())
                            .append(&mut unvalidated_moves_vec);
                        }
                        println!("unvalidated_moves {:?}", unvalidated_moves);
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
        }
       
        Ok(unvalidated_moves)
    }

    pub fn is_move_valid(&self, from_spot: &str, to_spot: &str, promotion_opt: Option<&str>)->Result<(MoveType), chess_errors::ChessErrors> {
        // first determine if piece at from is correct player.
        if let Ok(index) = chess_notation::notation_to_index(&from_spot) {
            if let Some(piece) = &self.state[index] {
                if piece.get_player() != self.player{
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
                if piece.get_player() == self.player{
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::PlayerPieceAlreadyThere(msg));
                }
            } 
        }
        if  promotion_opt.is_some() {
            //promotions are only valid from 8th rank for pawn
            let from_row = chess_notation::convert_row(from_spot)?;
            let to_row = chess_notation::convert_row(to_spot)?;
            if self.player == Player::White && to_row !=  0 &&  from_row != 1 {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
            }
            if self.player == Player::Black && to_row != 8 && from_row!= 7 {
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

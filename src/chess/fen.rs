use std::fmt;

use super::chess::{Chess, Piece, PieceType, FEN_INITIAL_STATE, INIT};

use super::chess_mcts::Player;
pub(crate) struct FenRecord {
    piece_placement_data: String,
    player: char,
    castling: String,
    en_passant_target: String,
    halfmove_clock: u8,
    full_move_number: u16,
}

impl From<&FenRecord> for Chess {
    fn from(fen_record: &FenRecord) -> Self {
        let mut rows_of_pieces: Vec<&str> = fen_record
            .piece_placement_data
            .split("/")
            .collect::<Vec<&str>>();

        let mut state: [Option<Piece>; 64] = [INIT; 64];
        let mut index: usize = 0;
        for row in rows_of_pieces {
            for piece_char in row.chars() {
                match piece_char {
                    'p' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackPawn,
                            moved: false,
                        });
                        index += 1;
                    }
                    'r' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackRook,
                            moved: false,
                        });
                        index += 1;
                    }
                    'n' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackKnight,
                            moved: false,
                        });
                        index += 1;
                    }
                    'b' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackBishop,
                            moved: false,
                        });
                        index += 1;
                    }
                    'q' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackQueen,
                            moved: false,
                        });
                        index += 1;
                    }
                    'k' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackKing,
                            moved: false,
                        });
                        index += 1;
                    }
                    'P' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhitePawn,
                            moved: false,
                        });
                        index += 1;
                    }
                    'R' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhiteRook,
                            moved: false,
                        });
                        index += 1;
                    }
                    'N' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhiteKnight,
                            moved: false,
                        });
                        index += 1;
                    }
                    'B' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhiteBishop,
                            moved: false,
                        });
                        index += 1;
                    }
                    'Q' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhiteQueen,
                            moved: false,
                        });
                        index += 1;
                    }
                    'K' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhiteKing,
                            moved: false,
                        });
                        index += 1;
                    }
                    num_empty_char => {
                        let num_empty = num_empty_char.to_string().parse::<u16>().unwrap();
                        for j in 0..num_empty {
                            state[index] = None;
                            index += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        Chess {
            state,
            full_move_number: fen_record.full_move_number,
            castling: fen_record.castling.clone(),
            halfmove_clock: fen_record.halfmove_clock,
            player: match fen_record.player {
                'w' => Player::White,
                _ => Player::Black,
            },
            en_passant_enabled: None,
        }
    }
}

impl Default for FenRecord {
    fn default() -> Self {
        Self {
            piece_placement_data: FEN_INITIAL_STATE.to_owned(),
            player: 'w',
            castling: "KQkq".to_owned(),
            en_passant_target: "-".to_owned(),
            halfmove_clock: Default::default(),
            full_move_number: Default::default(),
        }
    }
}
impl From<&String> for FenRecord {
    fn from(fen_string: &String) -> Self {
        let fields: Vec<&str> = fen_string.split(" ").collect();
        FenRecord {
            piece_placement_data: fields[0].to_owned(),
            player: fields[1].chars().next().unwrap(),
            castling: fields[2].to_owned(),
            en_passant_target: fields[3].to_owned(),
            halfmove_clock: fields[4].parse::<u8>().unwrap(),
            full_move_number: fields[5].parse::<u16>().unwrap(),
        }
    }
}

impl fmt::Display for FenRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {}",
            self.piece_placement_data,
            self.player,
            self.castling,
            self.en_passant_target,
            self.halfmove_clock,
            self.full_move_number
        )
    }
}

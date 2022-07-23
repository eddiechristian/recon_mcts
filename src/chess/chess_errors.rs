use std::fmt;

use std::error::Error;
use std::str::Utf8Error;
#[derive(Debug)]
pub enum ChessErrors {
    InvalidNotation(String),
    WrongPlayer(String),
    NoPiece(String),
    PlayerPieceAlreadyThere(String),
    PawnCantAttackForward(String),
    PawnCanOnlyAttackDiagonal(String),
    InvalidMove(String),
    InvalidPromotion(String),
    PieceBetween(String),
    Utf8Error,
}

impl From<Utf8Error> for ChessErrors {
    fn from(_error: Utf8Error) -> Self {
        ChessErrors::Utf8Error
    }
}

//Utf8Error

impl Error for ChessErrors {}

impl fmt::Display for ChessErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ChessErrors::InvalidNotation(x) => {
                write!(f, "{} is invalid chess notation", x)
            }
            ChessErrors::WrongPlayer(x) => {
                write!(f, "wrong player at {}", x)
            }
            ChessErrors::NoPiece(x) => {
                write!(f, "no piece at {}", x)
            }
            ChessErrors::PlayerPieceAlreadyThere(x) => {
                write!(f, "you have a  piece at {}", x)
            }
            ChessErrors::PawnCantAttackForward(x) => {
                write!(f, "pawn cant attack piece at {}", x)
            }
            ChessErrors::PawnCanOnlyAttackDiagonal(x) => {
                write!(f, "pawn cant move to {}", x)
            }
            ChessErrors::InvalidMove(x) => {
                write!(f, "piece cannot move to {}", x)
            }
            ChessErrors::InvalidPromotion(x) => {
                write!(f, "piece cannot be promoted to {}", x)
            }
            ChessErrors::PieceBetween(x) => {
                write!(
                    f,
                    "piece cannot move  because one of your pieces is at {}",
                    x
                )
            }
            _ => {
                write!(f, "ddddd")
            }
        }
    }
}

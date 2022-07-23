use std::collections::HashMap;

use super::{
    chess::Piece,
    chess_errors,
    chess_mcts::Player,
    chess_notation::{self},
};

pub(crate) fn get_pawn_unvalidated_moves(
    piece: &Piece,
    state: &[Option<Piece>; 64],
    spot: &str,
) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
    let mut unvalidated_moves = HashMap::new();

    let bounds = chess_notation::get_bounds(spot)?;
    if let Ok(index) = chess_notation::notation_to_index(&spot) {
        if let Some(piece) = &state[index] {
            if piece.get_player() == Player::White {
                let (top_opt, top_right_opt, top_left_opt) =
                    (bounds.top, bounds.top_right_diag, bounds.top_left_diag);
                if let Some(top_array) = top_opt {
                    let top = std::str::from_utf8(&top_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(top.to_string());
                    let next_bounds = chess_notation::get_bounds(top)?;
                    if let Some(next_top_array) = next_bounds.top {
                        let next_top = std::str::from_utf8(&next_top_array).unwrap();
                        unvalidated_moves
                            .entry(spot.to_owned())
                            .or_insert_with(|| Vec::new())
                            .push(next_top.to_string());
                    }
                }
                if let Some(top_right_array) = top_right_opt {
                    let top_right = std::str::from_utf8(&top_right_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(top_right.to_string());
                }
                if let Some(top_left_array) = top_left_opt {
                    let top_left = std::str::from_utf8(&top_left_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(top_left.to_string());
                }
            } else if piece.get_player() == Player::Black {
                let (bottom_opt, bottom_right_opt, bottom_left_opt) = (
                    bounds.bottom,
                    bounds.bottom_right_diag,
                    bounds.bottom_left_diag,
                );
                if let Some(bottom_array) = bottom_opt {
                    let bottom = std::str::from_utf8(&bottom_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(bottom.to_string());
                    let next_bounds = chess_notation::get_bounds(bottom)?;
                    if let Some(next_bottom_array) = next_bounds.bottom {
                        let next_bottom = std::str::from_utf8(&next_bottom_array).unwrap();
                        unvalidated_moves
                            .entry(spot.to_owned())
                            .or_insert_with(|| Vec::new())
                            .push(next_bottom.to_string());
                    }
                }
                if let Some(bottom_right_array) = bottom_right_opt {
                    let bottom_right = std::str::from_utf8(&bottom_right_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(bottom_right.to_string());
                }
                if let Some(bottom_left_array) = bottom_left_opt {
                    let bottom_left = std::str::from_utf8(&bottom_left_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(bottom_left.to_string());
                }
            }
        }
    }

    Ok(unvalidated_moves)
}

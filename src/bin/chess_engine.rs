use std::sync::Mutex;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use recon_mcts::chess::chess::{get_unvalidated_moves, FEN_INITIAL_STATE};

pub fn main() {
    let t0 = std::time::Instant::now();
    
    // for i in 0..1000 {
        let a = get_unvalidated_moves(FEN_INITIAL_STATE);
    // }
    println!("Elapsed: {:?}", t0.elapsed());
}

// pub fn main() {
//     let game = Chess {
//         max_move: MAX_MOVE,
//         rng: Mutex::new(rand::rngs::StdRng::seed_from_u64(0)),
//     };

//     let t = Tree::new(game, GetState, Player::P1, INIT);
//     let t0 = std::time::Instant::now();
//         // make 100 moves
//         for _ii in 0.. {
//             // for each move, expand the tree 100 times
//             for _jj in 0..100 {
//                 println!("{}:{} best action so far: {:?}", _ii, _jj, t.best_action());
//                 if t.step().is_none() {
//                     break;
//                 }
//             }

//             match t.apply_best_action() {
//                 Status::Action(a) => {
//                     println!("best action: {:?}", a);
//                     println!("root state: {:?}", t.get_root_info());
//                 }
//                 Status::Pending | Status::ActionWip(_) => unreachable!(),
//                 Status::Terminal => break,
//             }

//             let children = t.find_children_sorted_with_depth();

//             println!(
//                 "nnodes: {} depth: {}",
//                 children.len(),
//                 children.last().unwrap().1
//             );
//             assert_eq!(t.get_registry_nodes().len(), children.len());
//         }
//         println!("Elapsed: {:?}", t0.elapsed());
// }

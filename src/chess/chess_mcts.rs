//! Implementation of a `Monte Carlo Search Tree for Chess`
//! Its State will be represented as [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)

use crate::prelude::*;

use std::cell::Ref;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use rand::rngs::StdRng;
use rand::Rng;

use rand::SeedableRng;
use std::sync::Arc;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug)]
pub struct Score {
    player1: f64,
    player2: f64,
    visits_direct: AtomicUsize,
}

impl Score {
    fn fetch_add_visits_direct(&self, x: usize) -> usize {
        self.visits_direct.fetch_add(x, Ordering::Relaxed)
    }
}

impl Clone for Score {
    fn clone(&self) -> Self {
        let Score {
            ref player1,
            ref player2,
            ref visits_direct,
        } = self;
        Score {
            player1: *player1,
            player2: *player2,
            visits_direct: AtomicUsize::new(visits_direct.load(Ordering::Relaxed)),
        }
    }
}

#[doc(hidden)]
pub struct ActionIter {
    player: Player,
    available_moves: Vec<[char; 5]>,
}

impl ActionIter {
    fn new(player: Player, max: usize) -> Self {
        Self {
            player,
            available_moves: Vec::<[char; 5]>::with_capacity(10),
        }
    }
}

impl Iterator for ActionIter {
    type Item = (Player, [char; 5]);
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.available_moves.iter().next()?;
        Some((self.player.clone(), *a))
    }
}

#[derive(Debug)]
pub struct Chess {
    pub fen_state: String,
    // Include an rng to allow our implementation to include a random exploration incentive
    pub rng: Mutex<StdRng>,
}

impl GameDynamics for Chess {
    type Player = Player;
    type State = String;
    //let char_array = "text".chars().collect::<Vec<char>>();
    type Action = [char; 5];
    type Score = Score;
    type ActionIter = ActionIter;

    fn available_actions(
        &self,
        player: &Self::Player,
        _state: &Self::State,
    ) -> Option<Self::ActionIter> {
        // match player {
        //     Player::White => Some(ActionIter::new(Player::Black, self.max_move)),
        //     Player::Black => Some(ActionIter::new(Player::White, self.max_move)),
        // }
        todo!()
    }

    fn apply_action(&self, state: Self::State, action: &Self::Action) -> Option<Self::State> {
        todo!()
    }

    fn select_node<II, Q, A>(
        &self,
        parent_score: Option<&Self::Score>,
        parent_player: &Self::Player,
        parent_node_state: &Self::State,
        purpose: SelectNodeState,
        scores_and_actions: II,
    ) -> Self::Action
    where
        Self: Sized,
        II: Clone + IntoIterator<Item = (Q, A)>,
        Q: Deref<Target = Option<Self::Score>>,
        A: Deref<Target = Self::Action>,
    {
        todo!()
    }

    fn backprop_scores<II, Q>(
        &self,
        player: &Self::Player,
        score_current: Option<&Self::Score>,
        child_scores: II,
    ) -> Option<Self::Score>
    where
        Self: Sized,
        II: Clone + IntoIterator<Item = Q>,
        Q: Deref<Target = Self::Score>,
    {
        todo!()
    }

    fn score_leaf(
        &self,
        parent_score: Option<&Self::Score>,
        parent_player: &Self::Player,
        state: &Self::State,
    ) -> Option<Self::Score> {
        todo!()
    }
}

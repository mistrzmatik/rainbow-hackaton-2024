use crate::point_salad_server::{Card, GameState, VegtableHeld};
use crate::strategies::strategy::Strategy;

use super::mcts_game_state::MCTSGameState;

pub struct MonteCarloStrategy {
}

impl MonteCarloStrategy {
    pub fn new() -> MonteCarloStrategy {
        MonteCarloStrategy {
        }
    }
}

impl Strategy for MonteCarloStrategy {
    fn make_take_cards_move(&self, state: &GameState) -> Vec<String> {
        let initial_state = MCTSGameState::new();
        let mut mcts = MCTS::new(initial_state, 1000);
        // let root = NodeGame::new(initial_state);
        // let best_action = mcts::run(root, 1000);

        // println!("Najlepsza akcja w Point Salad: {}", best_action);

        let mut vec_of_strings = Vec::new();
        vec_of_strings.push("Hello".to_string());
        vec_of_strings
    }

    fn make_flip_move(&self, state: &GameState) -> Vec<String> {
        // let initial_state = StateGame::new();
        // let mut mcts = MCTS::new(initial_state, 1000);
        // let root = NodeGame::new(initial_state);
        // let best_action = mcts::run(root, 1000);

        // println!("Najlepsza akcja w Point Salad: {}", best_action);

        let mut vec_of_strings = Vec::new();
        vec_of_strings.push("Hello".to_string());
        vec_of_strings
    }
}
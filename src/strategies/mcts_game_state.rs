use std::collections::HashMap;

use crate::helpers::{drafted_cards};
use crate::point_salad_server::{Card, GameState};


#[derive(Clone)]
pub struct MCTSGameState {
    pub drafted_cards: Vec<Card>,
    pub player_vegetables: HashMap<String, i32>,
    pub player_point_cards: Vec<Card>,
}

impl MCTSGameState {
    // Create a new GameState with initial setup
    pub fn new(state: &GameState) -> MCTSGameState {
        MCTSGameState {
            drafted_cards: drafted_cards(state),
            player_vegetables: HashMap::new(),
            player_point_cards: vec![],
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.drafted_cards.len() < 36
    }

    pub fn available_actions(&self) -> Vec<String> {
        let mut actions = vec![];

        // // Add all available point cards as actions
        // for (i, card) in self.available_cards_count.iter().enumerate() {
        //     if card.point_condition.is_some() {
        //         actions.push(format!("Take point card {}", i));
        //     }
        // }

        // // Add all vegetable cards (take two vegetables)
        // if self.available_cards_count.len() >= 2 {
        //     actions.push("Take two vegetables".to_string());
        // }

        actions
    }

    // Apply an action to the game state, returning a new state
    pub fn apply_action(&self, action: &str) -> MCTSGameState {
        let mut new_state = self.clone();

        // if action.starts_with("Take point card") {
        //     let index: usize = action.split_whitespace().last().unwrap().parse().unwrap();
        //     let card = new_state.available_cards_count.remove(index);
        //     self.available_cards_count -= 1;
        //     new_state.player_point_cards.push(card);
        // } else if action == "Take two vegetables" {
        //     // For simplicity, just take the first two vegetable cards
        //     if let Some(card1) = new_state.available_cards_count.get(0) {
        //         if card1.vegetable_type.is_some() {
        //             let veg = card1.vegetable_type.clone().unwrap();
        //             *new_state.player_vegetables.entry(veg).or_insert(0) += 1;
        //         }
        //     }
        //     if let Some(card2) = new_state.available_cards_count.get(1) {
        //         if card2.vegetable_type.is_some() {
        //             let veg = card2.vegetable_type.clone().unwrap();
        //             *new_state.player_vegetables.entry(veg).or_insert(0) += 1;
        //         }
        //     }
        //     self.available_cards_count -= 1;
        //     new_state.available_cards_count.drain(0..2); // Remove two vegetables from the available cards
        // }

        new_state
    }

    // Calculate the player's score based on point cards and vegetables
    pub fn calculate_score(&self) -> i32 {
        let mut score = 0;

        // for point_card in &self.player_point_cards {
        //     if let Some(ref condition) = point_card.point_condition {
        //         for (veg, pts) in condition {
        //             let count = *self.player_vegetables.get(veg).unwrap_or(&0);
        //             score += pts * count;
        //         }
        //     }
        // }

        score
    }
}
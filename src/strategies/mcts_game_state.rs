use rand::Rng;
use std::collections::HashMap;
use std::f64::INFINITY;

#[derive(Clone)]
pub struct Card {
    // For simplicity, assume cards are either point cards or vegetable cards.
    // Vegetable cards: Each card represents a vegetable (e.g., tomato, lettuce, etc.)
    pub vegetable_type: Option<String>,
    // Point cards have a scoring condition based on vegetables
    pub point_condition: Option<HashMap<String, i32>>, // Vegetable type -> points
}

#[derive(Clone)]
pub struct MCTSGameState {
    pub available_cards: Vec<Card>,   // Cards on the table available for drafting
    pub player_vegetables: HashMap<String, i32>,  // Vegetables the player has
    pub player_point_cards: Vec<Card>,  // Point cards the player has
}

impl MCTSGameState {
    // Create a new GameState with initial setup
    pub fn new() -> MCTSGameState {
        MCTSGameState {
            available_cards: vec![], // Populate this with initial deck
            player_vegetables: HashMap::new(),
            player_point_cards: vec![],
        }
    }

    // Check if the game is over (e.g., all cards are drafted)
    pub fn is_terminal(&self) -> bool {
        self.available_cards.is_empty()
    }

    // Get available actions (take a point card or two vegetable cards)
    pub fn available_actions(&self) -> Vec<String> {
        let mut actions = vec![];

        // Add all available point cards as actions
        for (i, card) in self.available_cards.iter().enumerate() {
            if card.point_condition.is_some() {
                actions.push(format!("Take point card {}", i));
            }
        }

        // Add all vegetable cards (take two vegetables)
        if self.available_cards.len() >= 2 {
            actions.push("Take two vegetables".to_string());
        }

        actions
    }

    // Apply an action to the game state, returning a new state
    pub fn apply_action(&self, action: &str) -> MCTSGameState {
        let mut new_state = self.clone();

        if action.starts_with("Take point card") {
            let index: usize = action.split_whitespace().last().unwrap().parse().unwrap();
            let card = new_state.available_cards.remove(index);
            new_state.player_point_cards.push(card);
        } else if action == "Take two vegetables" {
            // For simplicity, just take the first two vegetable cards
            if let Some(card1) = new_state.available_cards.get(0) {
                if card1.vegetable_type.is_some() {
                    let veg = card1.vegetable_type.clone().unwrap();
                    *new_state.player_vegetables.entry(veg).or_insert(0) += 1;
                }
            }
            if let Some(card2) = new_state.available_cards.get(1) {
                if card2.vegetable_type.is_some() {
                    let veg = card2.vegetable_type.clone().unwrap();
                    *new_state.player_vegetables.entry(veg).or_insert(0) += 1;
                }
            }
            new_state.available_cards.drain(0..2); // Remove two vegetables from the available cards
        }

        new_state
    }

    // Calculate the player's score based on point cards and vegetables
    pub fn calculate_score(&self) -> i32 {
        let mut score = 0;

        for point_card in &self.player_point_cards {
            if let Some(ref condition) = point_card.point_condition {
                for (veg, pts) in condition {
                    let count = *self.player_vegetables.get(veg).unwrap_or(&0);
                    score += pts * count;
                }
            }
        }

        score
    }
}
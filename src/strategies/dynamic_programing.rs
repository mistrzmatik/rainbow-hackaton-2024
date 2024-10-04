use crate::point_salad_server::{GameState, Hand};
use crate::strategies::strategy::Strategy;

pub struct DynamicProgramingStrategy {}

impl DynamicProgramingStrategy {
    pub fn new() -> DynamicProgramingStrategy {
        DynamicProgramingStrategy {}
    }

    fn unwrap_hand(hand: &Option<Hand>) -> Hand {
        hand
            .clone()
            .unwrap_or(Hand {
                point_cards: vec![],
                vegetables: vec![],
            })
    }

    fn my_hand(&self, state: &GameState) -> Hand {
        DynamicProgramingStrategy::unwrap_hand(&state.your_hand.clone())
    }

    fn oponent_hand(&self, state: &GameState) -> Hand {
        DynamicProgramingStrategy::unwrap_hand(&state.opponents_hands.get(0).cloned())
    }
}

impl Strategy for DynamicProgramingStrategy {
    fn make_take_cards_move(&self, state: &GameState) -> Vec<String> {
        let my_hand = self.my_hand(state);
        let oponent_hand: Hand = self.oponent_hand(state);


        vec![]
    }

    fn make_flip_move(&self, state: &GameState) -> Vec<String> {
        todo!()
    }
}
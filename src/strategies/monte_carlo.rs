use crate::point_salad_server::GameState;
use crate::strategies::strategy::Strategy;

pub struct MonteCarloStrategy {}

impl MonteCarloStrategy {
    pub fn new() -> MonteCarloStrategy {
        MonteCarloStrategy {}
    }
}

impl Strategy for MonteCarloStrategy {
    fn make_take_cards_move(&mut self, state: &GameState) -> Vec<String> {
        todo!()
    }

    fn make_flip_move(&self, state: &GameState) -> Vec<String> {
        todo!()
    }
}
use crate::point_salad_server::GameState;
use crate::strategies::strategy::Strategy;

pub struct DynamicProgramingStrategy {}

impl DynamicProgramingStrategy {
    pub fn new() -> DynamicProgramingStrategy {
        DynamicProgramingStrategy {}
    }
}

impl Strategy for DynamicProgramingStrategy {
    fn make_take_cards_move(&self, state: &GameState) -> Vec<String> {
        todo!()
    }

    fn make_flip_move(&self, state: &GameState) -> Vec<String> {
        todo!()
    }
}
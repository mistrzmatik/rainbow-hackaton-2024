use crate::point_salad_server::{Card, GameState, Hand, VegtableHeld};
use crate::strategies::strategy::Strategy;
use crate::helpers;

pub struct DynamicProgramingStrategy {}

impl DynamicProgramingStrategy {
    pub fn new() -> DynamicProgramingStrategy {
        DynamicProgramingStrategy {}
    }
}

impl Strategy for DynamicProgramingStrategy {
    fn make_take_cards_move(&self, state: &GameState) -> Vec<String> {
        //let my_hand = helpers::my_hand(state);
       // let oponent_hand: Hand = helpers::oponent_hand(state);

        vec![]
    }

    fn make_flip_move(&self, state: &GameState) -> Vec<String> {
        todo!()
    }
}
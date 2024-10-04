use crate::point_salad_server::GameState;

pub trait Strategy {
    fn make_take_cards_move(&self, state: &GameState) -> Vec::<String>;
    fn make_flip_move(&self, state: &GameState) -> Vec::<String>;
}
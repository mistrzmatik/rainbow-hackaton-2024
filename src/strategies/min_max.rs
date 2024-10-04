use crate::point_salad_server::{Card, GameState, Hand, VegtableHeld};
use crate::points::calculate_points;
use crate::strategies::strategy::Strategy;

pub struct MinMaxStrategy {}

impl MinMaxStrategy {
    pub fn new() -> MinMaxStrategy {
        MinMaxStrategy {}
    }
}

impl Strategy for MinMaxStrategy {
    fn make_take_cards_move(&self, state: &GameState) -> Vec<String> {
        
        let your_hand = state.clone().your_hand.unwrap_or(Hand {
            point_cards: vec![],
            vegetables: vec![]
        });
        let opponents_hand = state.clone().opponents_hands.get(0).unwrap_or(&Hand {
            point_cards: vec![],
            vegetables: vec![]
        }).clone();
        
        let cards = [your_hand.point_cards.as_slice(), create_cards(your_hand.vegetables).as_slice()].concat();
        let opponent_cards = [opponents_hand.point_cards.as_slice(), create_cards(opponents_hand.vegetables).as_slice()].concat();
        
        let base_points = calculate_points(&cards, &opponent_cards);
        
        
        
        vec![]
    }

    fn make_flip_move(&self, state: &GameState) -> Vec<String> {
        vec![]
    }
}

fn create_cards(vegetables_held: Vec<VegtableHeld>) -> Vec<Card> {
    let mut carts = vec![];

    for vegetable_held in vegetables_held {
        for i in 0..vegetable_held.count {
            carts.push(Card {
                vegetable: vegetable_held.vegetable_type,
                point_type: 0,
                card_id: "uknown".to_string(),
                sum: None,
                other: None,
                fewest_most: None,
                even_odd: None,
                points_per_vegetable: None
            })
        }
    }

    carts
}
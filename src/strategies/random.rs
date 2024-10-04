use rand::Rng;
use rand::seq::IteratorRandom;
use crate::point_salad_server::{GameState, Market};
use crate::strategies::strategy::Strategy;

pub struct RandomStrategy {}

impl RandomStrategy {
    pub fn new() -> RandomStrategy {
        RandomStrategy {}
    }
}

impl Strategy for RandomStrategy {
    fn make_take_cards_move(&self, state: &GameState) -> Vec<String> {
        let any_point_cards_available = state
            .market
            .clone()
            .unwrap_or_else(|| Market {
                point_cards : vec![],
                vegetable_cards: vec![]
            })
            .point_cards
            .iter()
            .filter(|c| c.card_id != "")
            .any(|c| true);

        let cards: Vec<_> = if rand::thread_rng().gen_bool(0.5f64) && any_point_cards_available {
            state
                .market
                .clone()
                .unwrap_or_else(|| Market {
                    point_cards : vec![],
                    vegetable_cards: vec![]
                })
                .point_cards
                .iter()
                .filter(|c| c.card_id != "")
                .choose_multiple(&mut rand::thread_rng(), 1)
                .into_iter()
                .map(|c| c.card_id.to_string())
                .collect()
        } else {
            let vegetable_cards_count = state
                .market
                .clone()
                .unwrap_or_else(|| Market {
                    point_cards : vec![],
                    vegetable_cards: vec![]
                })
                .vegetable_cards
                .iter()
                .filter(|c| c.card_id != "")
                .count();

            state
                .market
                .clone()
                .unwrap_or_else(|| Market {
                    point_cards : vec![],
                    vegetable_cards: vec![]
                })
                .vegetable_cards
                .iter()
                .filter(|c| c.card_id != "")
                .choose_multiple(&mut rand::thread_rng(), 2)
                .into_iter()
                .map(|c| c.card_id.to_string())
                .collect()
        };

        cards
    }

    fn make_flip_move(&self, state: &GameState) -> Vec<String> {
        vec![]
    }
}
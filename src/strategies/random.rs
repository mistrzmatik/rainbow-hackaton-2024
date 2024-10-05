use rand::Rng;
use rand::seq::IteratorRandom;
use crate::point_salad_server::{GameState, Hand, Market};
use crate::strategies::strategy::Strategy;

pub struct RandomStrategy {
    take_point_card_probability: f64,
    flip_card_probability: f64
}

impl RandomStrategy {
    pub fn new() -> RandomStrategy {
        RandomStrategy {
            take_point_card_probability: 0.5,
            flip_card_probability: 0.02
        }
    }
}

impl Strategy for RandomStrategy {
    fn make_take_cards_move(&mut self, state: &GameState) -> Vec<String> {
        let market = state
            .market
            .clone()
            .unwrap_or(Market {
                point_cards : vec![],
                vegetable_cards: vec![]
            });

        let take_point_card = rand::thread_rng().gen_bool(self.take_point_card_probability);
        let any_point_cards_available = market
            .point_cards
            .iter()
            .filter(|c| c.card_id != "")
            .any(|c| true);

        let cards: Vec<_> = if take_point_card && any_point_cards_available {
            market
                .point_cards
                .iter()
                .filter(|c| c.card_id != "")
                .choose_multiple(&mut rand::thread_rng(), 1)
                .into_iter()
                .map(|c| c.card_id.to_string())
                .collect()
        } else {
            market
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
        let flip_card = rand::thread_rng().gen_bool(self.flip_card_probability);
        if flip_card {
            return vec![];
        }

        let cards_available_to_flip: Vec<String> = state.your_hand
            .clone()
            .unwrap_or(Hand {
                point_cards: vec![],
                vegetables: vec![]
            })
            .point_cards
            .iter()
            .filter(|c| c.card_id != "")
            .into_iter()
            .map(|c| c.card_id.to_string())
            .collect();
        if cards_available_to_flip.is_empty() {
            return vec![];
        }

        let cards = cards_available_to_flip
            .iter()
            .choose_multiple(&mut rand::thread_rng(), 1)
            .into_iter()
            .map(|c| c.to_string())
            .collect();

        cards
    }
}
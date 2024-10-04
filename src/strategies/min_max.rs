use crate::point_salad_server::{Card, GameState, Hand, Market, VegtableHeld};
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
        
        let your_hand = state.your_hand.clone().unwrap_or(Hand {
            point_cards: vec![],
            vegetables: vec![]
        });
        let opponents_hand = state.opponents_hands.clone().get(0).unwrap_or(&Hand {
            point_cards: vec![],
            vegetables: vec![]
        }).clone();
        
        let base_cards = [your_hand.point_cards.as_slice(), create_cards(your_hand.vegetables).as_slice()].concat();
        let base_opponent_cards = [opponents_hand.point_cards.as_slice(), create_cards(opponents_hand.vegetables).as_slice()].concat();
        
        let base_points = calculate_points(&base_cards, &base_opponent_cards);
        
        let market = state.market.clone().unwrap_or(Market {
            point_cards: vec![],
            vegetable_cards: vec![]
        });
        let market_point_cards: Vec<_> = market.point_cards.iter().filter(|c| c.card_id != "").collect();
        let market_vegetable_cards: Vec<_> = market.vegetable_cards.iter().filter(|c| c.card_id != "").collect();

        let mut max_points = -1000isize;
        let mut max_option = vec![];
        for market_point_card in market_point_cards {
            let possible_new_cards = vec![market_point_card.clone()];
            let possible_cards = [base_cards.as_slice(), possible_new_cards.as_slice()].concat();;
            
            let possible_points = calculate_points(&possible_cards, &base_opponent_cards);

            if possible_points > max_points {
                max_points = possible_points;
                max_option = possible_new_cards.iter().map(|c| c.card_id.to_string()).collect();
            }
        }
        
        if market_vegetable_cards.len() == 1 {
            let possible_new_cards = vec![market_vegetable_cards.first().unwrap().clone().clone()];
            let possible_cards = [base_cards.as_slice(), possible_new_cards.as_slice()].concat();

            let possible_points = calculate_points(&possible_cards, &base_opponent_cards);

            if possible_points > max_points {
                max_points = possible_points;
                max_option = possible_new_cards.iter().map(|c| c.card_id.to_string()).collect();
            }
        }

        for market_vegetable_card_one in &market_vegetable_cards {
            for market_vegetable_card_two in &market_vegetable_cards {
                if market_vegetable_card_one.card_id == market_vegetable_card_two.card_id {
                    continue;
                }
                
                let possible_new_cards = vec![market_vegetable_card_one.clone().clone(), market_vegetable_card_two.clone().clone()];
                let possible_cards = [base_cards.as_slice(), possible_new_cards.as_slice()].concat();

                let possible_points = calculate_points(&possible_cards, &base_opponent_cards);

                if possible_points > max_points {
                    max_points = possible_points;
                    max_option = possible_new_cards.iter().map(|c| c.card_id.to_string()).collect();
                }
            }
        }

        if max_option.is_empty() {
            println!("Lipa jest")
        }
        
        max_option
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
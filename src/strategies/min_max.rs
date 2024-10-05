use crate::point_salad_server::{Card, GameState, Hand, Market, VegtableHeld};
use crate::points::calculate_points;
use crate::strategies::strategy::Strategy;

pub struct MinMaxStrategy {
    is_flip_enabled: bool,
    flip_in_next_move: Option<String>
}

impl MinMaxStrategy {
    pub fn new(is_flip_enabled: bool) -> MinMaxStrategy {
        MinMaxStrategy {
            is_flip_enabled,
            flip_in_next_move: None
        }
    }
}

impl Strategy for MinMaxStrategy {
    fn make_take_cards_move(&mut self, state: &GameState) -> Vec<String> {

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

        let mut moves: Vec<Move> = vec![];
        
        let mut possible_flip_cards: Vec<Option<Card>> = vec![];
        possible_flip_cards.push(None);
        possible_flip_cards.extend(your_hand.point_cards.iter().map(|c| Some(c.clone())));

        for possible_flip_card in &possible_flip_cards {

            let mut base_cards_with_flip_change = base_cards.clone();
            
            if possible_flip_card.is_some() {
                let flip_card = possible_flip_card.clone().unwrap();
                let index = your_hand.point_cards.iter().position(|x| x.card_id == flip_card.card_id).unwrap();
                base_cards_with_flip_change.remove(index);
                base_cards_with_flip_change.push(create_card(flip_card.vegetable));
            }
            
            for market_point_card in &market_point_cards {
                let possible_new_cards = vec![market_point_card.clone().clone()];
                let possible_cards = [base_cards_with_flip_change.as_slice(), possible_new_cards.as_slice()].concat();

                let possible_points = calculate_points(&possible_cards, &base_opponent_cards);

                moves.push(Move {
                    cards_to_take: possible_new_cards.iter().map(|c| c.card_id.to_string()).collect(),
                    card_to_flip: if possible_flip_card.is_none() {None} else {Some(possible_flip_card.clone().unwrap().card_id.to_string())},
                    points: possible_points,
                });
                
                if possible_flip_card.is_none() {
                    let possible_new_cards = vec![create_card(market_point_card.vegetable)];
                    let possible_cards = [base_cards_with_flip_change.as_slice(), possible_new_cards.as_slice()].concat();

                    let possible_points = calculate_points(&possible_cards, &base_opponent_cards);

                    moves.push(Move {
                        cards_to_take: vec![market_point_card.card_id.to_string()],
                        card_to_flip: Some(market_point_card.card_id.to_string()),
                        points: possible_points,
                    });
                }
            }

            for market_vegetable_card_one in &market_vegetable_cards {
                for market_vegetable_card_two in &market_vegetable_cards {
                    if market_vegetable_card_one.card_id == market_vegetable_card_two.card_id {
                        continue;
                    }

                    let possible_new_cards = vec![market_vegetable_card_one.clone().clone(), market_vegetable_card_two.clone().clone()];
                    let possible_cards = [base_cards_with_flip_change.as_slice(), possible_new_cards.as_slice()].concat();

                    let possible_points = calculate_points(&possible_cards, &base_opponent_cards);

                    moves.push(Move {
                        cards_to_take: possible_new_cards.iter().map(|c| c.card_id.to_string()).collect(),
                        card_to_flip: if possible_flip_card.is_none() {None} else {Some(possible_flip_card.clone().unwrap().card_id.to_string())},
                        points: possible_points,
                    });
                }
            }

            if market_vegetable_cards.len() == 1 {
                let possible_new_cards = vec![market_vegetable_cards.first().unwrap().clone().clone()];
                let possible_cards = [base_cards_with_flip_change.as_slice(), possible_new_cards.as_slice()].concat();

                let possible_points = calculate_points(&possible_cards, &base_opponent_cards);

                moves.push(Move {
                    cards_to_take: possible_new_cards.iter().map(|c| c.card_id.to_string()).collect(),
                    card_to_flip: if possible_flip_card.is_none() {None} else {Some(possible_flip_card.clone().unwrap().card_id.to_string())},
                    points: possible_points,
                });
            }
        }
        
        moves.sort_by_key(|m| -m.points);
        let best_move = moves.first().unwrap();
        
        self.flip_in_next_move = best_move.card_to_flip.clone();
        best_move.cards_to_take.clone()
    }

    fn make_flip_move(&self, state: &GameState) -> Vec<String> {
        if !self.is_flip_enabled {
            return vec![]
        }
        
        if self.flip_in_next_move.is_some(){
            return vec![self.flip_in_next_move.clone().unwrap()]
        }

        vec![]
    }
}

pub struct Move {
    cards_to_take: Vec<String>,
    card_to_flip: Option<String>,
    points: isize
}

fn create_cards(vegetables_held: Vec<VegtableHeld>) -> Vec<Card> {
    let mut carts = vec![];

    for vegetable_held in vegetables_held {
        for i in 0..vegetable_held.count {
            carts.push(create_card(vegetable_held.vegetable_type))
        }
    }

    carts
}

fn create_card(vegetable_type: i32) -> Card {
    Card {
        vegetable: vegetable_type,
        point_type: 0,
        card_id: "unknown".to_string(),
        sum: None,
        other: None,
        fewest_most: None,
        even_odd: None,
        points_per_vegetable: None
    }
}
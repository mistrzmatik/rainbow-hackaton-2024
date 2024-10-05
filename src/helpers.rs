use crate::point_salad_server::{Card, GameState, Hand, Market, VegtableHeld};

pub fn market_cards(state: &GameState) -> Vec<Card> {
    let mut cards = vec![];

    let market = unwrap_market(&state.market);
    cards.extend(market.point_cards.clone());
    cards.extend(market.vegetable_cards.clone());

    cards
}

pub fn drafted_cards(state: &GameState) -> Vec<Card> {
    market_cards(state)
        + hand_cards(my_hand(state))
        + hand_cards(oponent_hand(state))
}

pub fn unwrap_hand(hand: &Option<Hand>) -> Hand {
    hand
        .clone()
        .unwrap_or(Hand {
            point_cards: vec![],
            vegetables: vec![],
        })
}

pub fn unwrap_market(market: &Option<Market>) -> Market {
    market.unwrap_or(Market {
        point_cards: vec![],
        vegetable_cards: vec![],
    })
}

pub fn my_hand(state: &GameState) -> Hand {
    unwrap_hand(&state.your_hand.clone())
}

pub fn oponent_hand(state: &GameState) -> Hand {
    unwrap_hand(&state.opponents_hands.get(0).cloned())
}

fn hand_cards(hand: Hand) -> Vec<Card> {
    let mut cards = vec![];

    cards.extend(hand.point_cards.clone());
    cards.extend(create_cards(hand.vegetables.clone()));

    cards
}

fn create_cards(vegetables_held: Vec<VegtableHeld>) -> Vec<Card> {
    let mut carts = vec![];

    for vegetable_held in vegetables_held {
        for _i in 0..vegetable_held.count {
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
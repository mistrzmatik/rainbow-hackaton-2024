use crate::point_salad_server::{Market, Card};

pub struct BetterGameState
{
    pub full_cards: Vec<Card>,
    pub taken_cards: Vec<Card>,
    pub available_cards: Vec<Card>,
    pub move_to_make: i32,
    pub your_cards: Vec<Card>,
    pub opponents_cards: Vec<Card>,
    pub market: Market,
}

impl BetterGameState {
    pub fn new(full_cards: Vec<Card>) -> BetterGameState {
        BetterGameState {
            full_cards: full_cards.iter().cloned().collect(),
            taken_cards: vec![],
            available_cards: vec![],
            move_to_make: 0,
            your_cards: vec![],
            opponents_cards: vec![],
            market: crate::point_salad_server::Market {
                point_cards: vec![],
                vegetable_cards: vec![],
            },
        }
    }
}
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::point_salad_server::{Card, GameState, Hand, Market, VegtableHeld};
use crate::points::calculate_points;
use crate::strategies::strategy::Strategy;

pub struct MinMaxStrategy {
    calc_type: i32,
    flip_in_next_move: Option<String>
}

impl MinMaxStrategy {
    pub fn new(calc_type: i32) -> MinMaxStrategy {
        MinMaxStrategy {
            calc_type,
            flip_in_next_move: None
        }
    }
}

impl Strategy for MinMaxStrategy {
    fn make_take_cards_move(&mut self, state: &GameState) -> Vec<String> {

        let market = state.market.clone().unwrap_or(Market {
            point_cards: vec![],
            vegetable_cards: vec![]
        });
        let market_point_cards: Vec<_> = market.point_cards.iter().filter(|c| c.card_id != "").collect();
        let market_vegetable_cards: Vec<_> = market.vegetable_cards.iter().filter(|c| c.card_id != "").collect();

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

        let mut possible_flip_cards: Vec<Option<Card>> = vec![];
        possible_flip_cards.push(None);
        possible_flip_cards.extend(your_hand.point_cards.iter().map(|c| Some(c.clone())));

        let moves_for_me: Vec<Move> = get_moves(&base_cards, &base_opponent_cards, &possible_flip_cards, &market_point_cards, &market_vegetable_cards);

        let mut possible_flip_cards_for_opponent: Vec<Option<Card>> = vec![];
        possible_flip_cards_for_opponent.push(None);
        possible_flip_cards_for_opponent.extend(opponents_hand.point_cards.iter().map(|c| Some(c.clone())));

        let mut moves_diffs: Vec<MoveDiff> = vec![];
        for move_for_me in moves_for_me {
            let mut market_point_cards_copy: Vec<_> = market_point_cards.clone();
            let mut market_vegetable_cards_copy: Vec<_> = market_vegetable_cards.clone();

            let mut base_cards_copy:  Vec<_>  = base_cards.clone();
            for taken_card in move_for_me.cards_to_take.clone() {
                let index = market_point_cards_copy.iter().position(|x| x.card_id == taken_card.card_id);
                if let Some(i) = index {
                    market_point_cards_copy.remove(i);
                }

                let index = market_vegetable_cards_copy.iter().position(|x| x.card_id == taken_card.card_id);
                if let Some(i) = index {
                    market_vegetable_cards_copy.remove(i);
                }

                base_cards_copy.push(taken_card)
            }
            if move_for_me.card_to_flip.is_some() {
                let card_to_flip = move_for_me.card_to_flip.clone().unwrap();

                let index = base_cards_copy.iter().position(|x| x.card_id == card_to_flip.card_id).unwrap();
                base_cards_copy.remove(index);
                base_cards_copy.push(create_card(card_to_flip.vegetable, &card_to_flip.card_id));
            }

            let mut moves_for_opponent: Vec<Move> = get_moves(&base_opponent_cards, &base_cards_copy, &possible_flip_cards_for_opponent, &market_point_cards_copy, &market_vegetable_cards_copy);
            moves_for_opponent.sort_by_key(|m| -m.points_diff);

            let best_move_for_opponent = moves_for_opponent.first();
            let best_move_for_opponent_point_diff = best_move_for_opponent.unwrap_or(&Move {
                points_diff: 0,
                card_to_flip: None,
                cards_to_take: vec![]
            }).points_diff;
            // 5 8, 5 - 8 = - 3,  8 + (-3) = 5, wynik 5-5  = 0
            let diff = match self.calc_type {
                1 => move_for_me.points_diff - best_move_for_opponent_point_diff,
                _ => move_for_me.points_diff - best_move_for_opponent_point_diff
            };
            moves_diffs.push(MoveDiff {
                my_move: move_for_me,
                higher_diff_for_opponent: best_move_for_opponent_point_diff,
                points_diff: diff
            })
        }
        moves_diffs.sort_by_key(|m| -m.points_diff);

        let best_moves_diffs = moves_diffs.first().unwrap();

        /*println!(
            "Najlepszy ruch: {:?} = {:?} - {:?}",
            best_moves_diffs.points_diff,
            best_moves_diffs.my_move.points_diff,
            best_moves_diffs.higher_diff_for_opponent
        );*/

        self.flip_in_next_move = best_moves_diffs.my_move.clone().card_to_flip.map(|c| c.card_id);
        best_moves_diffs.my_move.cards_to_take.iter().map(|c| c.card_id.to_string()).collect()
    }

    fn make_flip_move(&self, state: &GameState) -> Vec<String> {
        if self.flip_in_next_move.is_some(){
            return vec![self.flip_in_next_move.clone().unwrap()]
        }

        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    cards_to_take: Vec<Card>,
    card_to_flip: Option<Card>,
    points_diff: isize
}

#[derive(Debug)]
pub struct MoveDiff {
    my_move: Move,
    higher_diff_for_opponent: isize,
    points_diff: isize
}

fn get_moves(base_cards: &Vec<Card>, base_opponent_cards: &Vec<Card>, possible_flip_cards: &Vec<Option<Card>>, market_point_cards: &Vec<&Card>, market_vegetable_cards: &Vec<&Card>) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let base_points = calculate_points(&base_cards, &base_opponent_cards);
    for possible_flip_card in possible_flip_cards {
        let mut base_cards_with_flip_change = base_cards.clone();

        if possible_flip_card.is_some() {
            let flip_card = possible_flip_card.clone().unwrap();
            let index = base_cards_with_flip_change.iter().position(|x| x.card_id == flip_card.card_id).unwrap();
            base_cards_with_flip_change.remove(index);
            base_cards_with_flip_change.push(create_card(flip_card.vegetable, &flip_card.card_id));
        }

        for market_point_card in market_point_cards {
            let possible_new_cards = vec![market_point_card.clone().clone()];
            let possible_cards = [base_cards_with_flip_change.as_slice(), possible_new_cards.as_slice()].concat();

            let possible_points = calculate_points(&possible_cards, &base_opponent_cards);

            moves.push(Move {
                cards_to_take: possible_new_cards,
                card_to_flip: possible_flip_card.clone(),
                points_diff: possible_points - base_points,
            });

            if possible_flip_card.is_none() {
                let possible_new_cards = vec![create_card(market_point_card.vegetable, &market_point_card.card_id)];
                let possible_cards = [base_cards_with_flip_change.as_slice(), possible_new_cards.as_slice()].concat();

                let possible_points = calculate_points(&possible_cards, &base_opponent_cards);

                moves.push(Move {
                    cards_to_take: possible_new_cards,
                    card_to_flip: Some(create_card(market_point_card.vegetable, &market_point_card.card_id)),
                    points_diff: possible_points - base_points,
                });
            }
        }

        for market_vegetable_card_one in market_vegetable_cards {
            for market_vegetable_card_two in market_vegetable_cards {
                if market_vegetable_card_one.card_id == market_vegetable_card_two.card_id {
                    continue;
                }

                let possible_new_cards = vec![market_vegetable_card_one.clone().clone(), market_vegetable_card_two.clone().clone()];
                let possible_cards = [base_cards_with_flip_change.as_slice(), possible_new_cards.as_slice()].concat();

                let possible_points = calculate_points(&possible_cards, &base_opponent_cards);

                moves.push(Move {
                    cards_to_take: possible_new_cards,
                    card_to_flip: possible_flip_card.clone(),
                    points_diff: possible_points - base_points,
                });
            }
        }

        if market_vegetable_cards.len() == 1 {
            let possible_new_cards = vec![market_vegetable_cards.first().unwrap().clone().clone()];
            let possible_cards = [base_cards_with_flip_change.as_slice(), possible_new_cards.as_slice()].concat();

            let possible_points = calculate_points(&possible_cards, &base_opponent_cards);

            moves.push(Move {
                cards_to_take: possible_new_cards,
                card_to_flip: possible_flip_card.clone(),
                points_diff: possible_points - base_points,
            });
        }
    }

    moves
}

fn create_cards(vegetables_held: Vec<VegtableHeld>) -> Vec<Card> {
    let mut carts = vec![];

    for vegetable_held in vegetables_held {
        for i in 0..vegetable_held.count {
            let s: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();

            carts.push(create_card(vegetable_held.vegetable_type, &s))
        }
    }

    carts
}

fn create_card(vegetable_type: i32, card_id: &str) -> Card {
    Card {
        vegetable: vegetable_type,
        point_type: 0,
        card_id: card_id.to_string(),
        sum: None,
        other: None,
        fewest_most: None,
        even_odd: None,
        points_per_vegetable: None
    }
}
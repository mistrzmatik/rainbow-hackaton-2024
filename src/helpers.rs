use crate::{point_salad_server::{Card, GameState, Hand, VegtableHeld}};

pub fn unwrap_hand(hand: &Option<Hand>) -> Hand {
    hand
        .clone()
        .unwrap_or(Hand {
            point_cards: vec![],
            vegetables: vec![],
        })
}

pub fn my_hand(state: &GameState) -> Hand {
    unwrap_hand(&state.your_hand.clone())
}

pub fn oponent_hand(state: &GameState) -> Hand {
    unwrap_hand(&state.opponents_hands.get(0).cloned())
}

fn count_points(card: Card, vegetables: Vec<VegtableHeld>) -> i32 {
    let mut total_points = 0;

    match card.point_type {
        1 => {
            if let Some(points_per_vegetable) = card.points_per_vegetable {
                total_points += vegetables.iter().filter(|v| v.vegetable_type == card.vegetable).count();
            }
        }
        2 => {
            if let Some(sum) = card.sum {
                let total_vegetables = vegetables
                    .iter()
                    .filter(|v| sum.vegetables.contains(&v.vegetable_type))
                    .min
                total_points += total_vegetables * sum.points;
            }
        }
        3 => {
            if let Some(even_odd) = card.even_odd {
                if let Some(veg) = vegetables.iter().find(|v| v.vegetable_type == card.vegetable) {
                    if veg.count % 2 == 0 {
                        total_points += card.points_per_vegetable.unwrap().points.eve;
                    } else {
                        total_points += card.points_per_vegetable.unwrap().points;
                    }
                }
            }
        }
        4 => {
            if let Some(fewest_most) = card.fewest_most {
                let min_count = vegetables.iter().map(|v| v.count).min().unwrap_or(0);
                let max_count = vegetables.iter().map(|v| v.count).max().unwrap_or(0);
                
                if let Some(veg) = vegetables.iter().find(|v| v.vegetable_type == card.vegetable) {
                    if veg.count == min_count {
                        total_points += fewest_most.fewest_points;
                    } else if veg.count == max_count {
                        total_points += fewest_most.most_points;
                    }
                }
            }
        }
        5 => {
            if let Some(other) = card.other {
                total_points += other.points;
            }
        }
        _ => {}
    }

    total_points
}


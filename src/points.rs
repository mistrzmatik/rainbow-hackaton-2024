use std::any::{Any, TypeId};
use crate::point_salad_server::{Card, PointType};

pub fn calculate_points(cards: Vec<Card>) -> usize {
    let point_carts: Vec<_> = cards.iter().filter(|c| c.point_type != 0).collect();
    let vegetables_carts: Vec<_> =  cards.iter().filter(|c| c.point_type == 0).collect();
    
    let mut points = 0;
    for point_cart in point_carts {
        points += calculate_points_for_card(point_cart, &vegetables_carts);
    }

    points
}

const BAD_POINT_TYPE: i32 = 0;
/// PointsPerVegetable 
const POINTS_PER_VEGETABLE_ONE: i32 = 1;
const POINTS_PER_VEGETABLE_TWO: i32 = 2;
const POINTS_PER_VEGETABLE_THREE: i32 = 3;
/// Sum
const SUM_TWO: i32 = 4;
const SUM_THREE: i32 = 5;
/// EvenOdd
const EVEN_ODD: i32 = 6;
/// FewestMost
const FEWEST: i32 = 7;
const MOST: i32 = 8;
/// Other
const MOST_TOTAL: i32 = 9;
const FEWEST_TOTAL: i32 = 10;
const COMPLETE_SET: i32 = 11;
const AT_LEAST_TWO: i32 = 12;
const AT_LEAST_THREE: i32 = 13;
const MISSING_VEGETABLE: i32 = 14;

fn calculate_points_for_card(card: &Card, vegetables_carts: &Vec<&Card>) -> usize {
    if card.points_per_vegetable.is_some() {
        let mut points = 0;
        for point in card.clone().points_per_vegetable.unwrap().points {
            points += vegetables_carts.iter().filter(|c| c.vegetable == point.vegetable).count() * points;
        }
        return points;
    }
    
    if card.sum.is_some() {
        let mut min_count = 10000;
        for vegetable in card.clone().sum.unwrap().vegetables {
            let vegetable_count = vegetables_carts.iter().filter(|c| c.vegetable == vegetable).count();
            if vegetable_count < min_count {
                min_count = vegetable_count
            }
        }
        return min_count * card.clone().sum.unwrap().points as usize;
    }
    
    if card.even_odd.is_some() {
        let even_odd = card.even_odd.unwrap();
        let vegetable_count = vegetables_carts.iter().filter(|c| c.vegetable == even_odd.vegetable).count();
        if vegetable_count % 2 == 0 {
            return even_odd.even as usize;
        } else {
            return even_odd.odd as usize;
        }
    }
    
    if card.fewest_most.is_some() {
        let fewest_most = card.fewest_most.unwrap();
        //fewest_most.points
    }
    
    0
}
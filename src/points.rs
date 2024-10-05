use crate::point_salad_server::{Card, EvenOdd, FewestMost, Other, PointsPerVegetable, Sum, VegetableType};

pub fn calculate_points(cards: &Vec<Card>, opponent_cards: &Vec<Card>) -> isize {
    let point_carts: Vec<_> = cards.iter().filter(|c| c.point_type != 0).collect();
    let vegetables_carts: Vec<_> = cards.iter().filter(|c| c.point_type == 0).collect();
    let opponent_vegetables_carts: Vec<_> = opponent_cards.iter().filter(|c| c.point_type == 0).collect();
    
    let mut points = 0;
    for point_cart in point_carts {
        points += calculate_points_for_card(point_cart, &vegetables_carts, &opponent_vegetables_carts);
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

fn vegetable_counts(vegetables_carts: &Vec<&Card>) -> Vec<usize> {
    vec![
        vegetables_carts.iter().filter(|c| c.vegetable == VegetableType::Cabbage as i32).count(),
        vegetables_carts.iter().filter(|c| c.vegetable == VegetableType::Carrot as i32).count(),
        vegetables_carts.iter().filter(|c| c.vegetable == VegetableType::Lettuce as i32).count(),
        vegetables_carts.iter().filter(|c| c.vegetable == VegetableType::Onion as i32).count(),
        vegetables_carts.iter().filter(|c| c.vegetable == VegetableType::Pepper as i32).count(),
        vegetables_carts.iter().filter(|c| c.vegetable == VegetableType::Tomato as i32).count()
    ]
}

fn points_per_vegetable_score(points_per_vegetable: PointsPerVegetable, vegetables_carts: &Vec<&Card>) -> isize {
    let mut points = 0;
        for point in points_per_vegetable.points {
        points += (vegetables_carts.iter().filter(|c| c.vegetable == point.vegetable).count() as isize) * (point.points as isize);
    }
    return points as isize;
}

fn sum_score(sum: Sum, vegetables_carts: &Vec<&Card>) -> isize {
    let first_vegetable = sum.vegetables.get(0).unwrap_or(&0);
    let the_same_vegetable = sum.vegetables.iter().all(|v| v == first_vegetable);
    return if the_same_vegetable {
        let vegetable = first_vegetable.clone();
        let vegetable_count = vegetables_carts.iter().filter(|c| c.vegetable == vegetable).count();
        (vegetable_count as isize / sum.vegetables.len() as isize) * sum.points as isize
    } else {
        let mut min_count = 10000;
        for vegetable in sum.vegetables {
            let vegetable_count = vegetables_carts.iter().filter(|c| c.vegetable == vegetable).count();
            if vegetable_count < min_count {
                min_count = vegetable_count
            }
        }
        min_count as isize * sum.points as isize
    }
}

fn even_odd_score(even_odd: EvenOdd, vegetables_carts: &Vec<&Card>) -> isize {
    let vegetable_count = vegetables_carts.iter().filter(|c| c.vegetable == even_odd.vegetable).count();
    if vegetable_count == 0 || vegetable_count % 2 != 0 {
        return even_odd.odd as isize;
    } else {
        return even_odd.even as isize;
    }
}

fn fewest_most_score(fewest_most: FewestMost, card: &Card, vegetables_carts: &Vec<&Card>, opponent_vegetables_carts: &Vec<&Card>) -> isize {
    let vegetable_count = vegetables_carts.iter().filter(|c| c.vegetable == fewest_most.vegetable).count();
    let opponent_vegetable_count = opponent_vegetables_carts.iter().filter(|c| c.vegetable == fewest_most.vegetable).count();
    if card.point_type == MOST {
        if vegetable_count >= opponent_vegetable_count {
            return fewest_most.points as isize;
        }
    } else {
        if vegetable_count <= opponent_vegetable_count {
            return fewest_most.points as isize;
        }
    }
    0
}

fn other_most_total_score(other: Other, total_vegetable_count: usize, total_opponent_vegetable_count: usize) -> isize {
    if total_vegetable_count >= total_opponent_vegetable_count {
        return other.points as isize;
    }
    0
}

fn other_fewest_total_score(other: Other, total_vegetable_count: usize, total_opponent_vegetable_count: usize) -> isize {
    if total_vegetable_count <= total_opponent_vegetable_count {
        return other.points as isize;
    }
    0
}

fn other_at_least(other: Other, card: &Card, vegetables_carts: &Vec<&Card>) -> isize {
    let vegetable_counts = vegetable_counts(vegetables_carts);

    let at_least = if card.point_type == AT_LEAST_TWO { 2usize } else { 3usize };
    let at_least_count = vegetable_counts.iter().filter(|c| c >= &&at_least).count();
    return at_least_count as isize * other.points as isize;
}

fn other_complete_set(other: Other, vegetables_carts: &Vec<&Card>) -> isize {
    let vegetable_counts = vegetable_counts(vegetables_carts);

    let min: &usize = vegetable_counts.iter().min().unwrap_or(&0);
    return min.clone() as isize * other.points as isize;
}

fn other_missing_vegetable(other: Other, vegetables_carts: &Vec<&Card>) -> isize {
    let vegetable_counts = vegetable_counts(vegetables_carts);

    let zero_count = vegetable_counts.iter().filter(|c| c == &&0).count();
    return zero_count as isize * other.points as isize;
}

fn other_score(other: Other, card: &Card, vegetables_carts: &Vec<&Card>, opponent_vegetables_carts: &Vec<&Card>) -> isize {
    let total_vegetable_count = vegetables_carts.len();
    let total_opponent_vegetable_count = opponent_vegetables_carts.len();
    
    match card.point_type {
        MOST_TOTAL => other_most_total_score(other, total_vegetable_count, total_opponent_vegetable_count),
        FEWEST_TOTAL => other_fewest_total_score(other, total_vegetable_count, total_opponent_vegetable_count),
        COMPLETE_SET => other_complete_set(other, vegetables_carts),
        AT_LEAST_TWO | 
        AT_LEAST_THREE => other_at_least(other, card, vegetables_carts),
        MISSING_VEGETABLE => other_missing_vegetable(other, vegetables_carts),
        _ => return 0
    }
}

fn calculate_points_for_card(card: &Card, vegetables_carts: &Vec<&Card>, opponent_vegetables_carts: &Vec<&Card>) -> isize {
    if let Some(points_per_vegetable) = card.points_per_vegetable.clone() {
        return points_per_vegetable_score(points_per_vegetable, vegetables_carts);
    }
    
    if let Some(sum) = card.sum.clone() {
        return sum_score(sum, vegetables_carts)
    }

    if let Some(even_odd) = card.even_odd.clone() {
        return even_odd_score(even_odd, vegetables_carts)
    }

    if let Some(fewest_most) = card.fewest_most.clone() {
        return fewest_most_score(fewest_most, card, vegetables_carts, opponent_vegetables_carts)
    }

    if let Some(other) = card.other.clone() {
        return other_score(other, card, vegetables_carts, opponent_vegetables_carts)
    }
    
    0
}

#[cfg(test)]
mod tests {
    use crate::point_salad_server::VegetablePoints;

    use super::*;

    #[test]
    fn test_vegetable_counts() {
        let cabbage_card = Card { vegetable: VegetableType::Cabbage as i32, ..Default::default() };
        let carrot_card = Card { vegetable: VegetableType::Carrot as i32, ..Default::default() };
        let lettuce_card = Card { vegetable: VegetableType::Lettuce as i32, ..Default::default() };
        let vegetables_carts = vec![&cabbage_card, &carrot_card, &lettuce_card, &cabbage_card];

        let counts = vegetable_counts(&vegetables_carts);
        assert_eq!(counts, vec![2, 1, 1, 0, 0, 0]);
    }

    #[test]
    fn test_points_per_vegetable_score() { // -- to fix
        let cabbage_card = Card { vegetable: VegetableType::Cabbage as i32, ..Default::default() };
        let carrot_card = Card { vegetable: VegetableType::Carrot as i32, ..Default::default() };
        let points_per_vegetable = PointsPerVegetable {
            points: vec![
                VegetablePoints { vegetable: VegetableType::Cabbage as i32, points: 3 },
                VegetablePoints { vegetable: VegetableType::Carrot as i32, points: 2 },
            ],
        };

        let vegetables_carts = vec![&cabbage_card, &carrot_card, &cabbage_card];
        let points = points_per_vegetable_score(points_per_vegetable, &vegetables_carts);
        assert_eq!(points, 8); // (2 * 3) + (1 * 2) = 6 + 2 = 8
    }

    #[test]
    fn test_sum_score() {
        let cabbage_card = Card { vegetable: VegetableType::Cabbage as i32, ..Default::default() };
        let carrot_card = Card { vegetable: VegetableType::Carrot as i32, ..Default::default() };
        let sum = Sum {
            vegetables: vec![VegetableType::Cabbage as i32, VegetableType::Cabbage as i32],
            points: 5,
        };

        let vegetables_carts = vec![&cabbage_card, &carrot_card, &cabbage_card];
        let score = sum_score(sum, &vegetables_carts);
        assert_eq!(score, 5); // 2 cabbages, same type, so 1 * 5 = 5
    }

    #[test]
    fn test_even_odd_score() {
        let cabbage_card = Card { vegetable: VegetableType::Cabbage as i32, ..Default::default() };
        let carrot_card = Card { vegetable: VegetableType::Carrot as i32, ..Default::default() };
        let even_odd = EvenOdd {
            vegetable: VegetableType::Cabbage as i32,
            even: 10,
            odd: 5,
        };

        let vegetables_carts = vec![&cabbage_card, &cabbage_card];
        let score = even_odd_score(even_odd, &vegetables_carts);
        assert_eq!(score, 10); // 2 cabbages, even number, so 10 points

        let vegetables_carts = vec![&cabbage_card];
        let score = even_odd_score(even_odd, &vegetables_carts);
        assert_eq!(score, 5); // 1 cabbage, odd number, so 5 points
    }

    #[test]
    fn test_fewest_most_score() {
        let cabbage_card = Card { vegetable: VegetableType::Cabbage as i32, ..Default::default() };
        let carrot_card = Card { vegetable: VegetableType::Carrot as i32, ..Default::default() };
        let fewest_most = FewestMost {
            vegetable: VegetableType::Cabbage as i32,
            points: 3,
        };

        let card = Card {
            vegetable: VegetableType::Cabbage as i32,
            point_type: MOST,
            ..Default::default()
        };

        let vegetables_carts = vec![&cabbage_card, &cabbage_card];
        let opponent_vegetables_carts = vec![&carrot_card];
        let score = fewest_most_score(fewest_most, &card, &vegetables_carts, &opponent_vegetables_carts);
        assert_eq!(score, 3); // player has 2 cabbages, opponent has 0 cabbages, so MOST is true, 3 points
    }

    #[test]
    fn test_other_score_most_total() {
        let cabbage_card = Card { vegetable: VegetableType::Cabbage as i32, ..Default::default() };
        let carrot_card = Card { vegetable: VegetableType::Carrot as i32, ..Default::default() };
        let other = Other { points: 4 };

        let card = Card {
            vegetable: VegetableType::Cabbage as i32,
            point_type: MOST_TOTAL,
            ..Default::default()
        };

        let vegetables_carts = vec![&cabbage_card, &carrot_card];
        let opponent_vegetables_carts = vec![&cabbage_card];
        let score = other_score(other, &card, &vegetables_carts, &opponent_vegetables_carts);
        assert_eq!(score, 4); // player has more total cards, gets points
    }

    #[test]
    fn test_calculate_points_for_card_points_per_vegetable() { // -- to fix
        let cabbage_card = Card { vegetable: VegetableType::Cabbage as i32, ..Default::default() };
        let carrot_card = Card { vegetable: VegetableType::Carrot as i32, ..Default::default() };
        let points_per_vegetable = PointsPerVegetable {
            points: vec![VegetablePoints { vegetable: VegetableType::Cabbage as i32, points: 2 }],
        };
        let card = Card {
            points_per_vegetable: Some(points_per_vegetable),
            ..Default::default()
        };

        let vegetables_carts = vec![&cabbage_card, &carrot_card, &cabbage_card];
        let opponent_vegetables_carts = vec![];
        let points = calculate_points_for_card(&card, &vegetables_carts, &opponent_vegetables_carts);
        assert_eq!(points, 4); // 2 cabbages, 2 points each
    }
}

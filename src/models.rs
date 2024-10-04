use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    card_id: String,
    vegetable: String,
    point_type: String,
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Details {
    PointsPerVegetable(Vec<VegetablePoints>),
    Sum(SumDetails),
    EvenOdd(EvenOddDetails),
    FewestMost(FewestMostDetails),
    Other(OtherDetails),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VegetablePoints {
    vegetable: String,
    points: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SumDetails {
    vegetables: Vec<String>,
    points: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvenOddDetails {
    vegetable: String,
    even: i32,
    odd: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FewestMostDetails {
    vegetable: String,
    points: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherDetails {
    points: i32,
}

pub enum PointType {
    PointsPerVegetable,
    Sum,
    EvenOdd,
    FewestMost,
    Other,
}

#[derive(Clone, Copy, Debug)]
pub enum Vegetable {
    Onion,
    Tomato,
    Carrot,
    Lettuce,
    Pepper,
    Cabbage,
}

impl Vegetable {
    pub fn list() -> Vec<Vegetable> {
        vec![
            Vegetable::Onion,
            Vegetable::Tomato,
            Vegetable::Carrot,
            Vegetable::Lettuce,
            Vegetable::Pepper,
            Vegetable::Cabbage,
        ]
    }
}
use core::panic;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::point_salad_server::{Card, EvenOdd, FewestMost, Other, PointsPerVegetable, Sum, VegetablePoints, VegetableType};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct CardRaw {
    card_id: String,
    vegetable: String,
    point_type: String,
    details: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointsPerVegetableRaw {
    points: Vec<VegetablePointsRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SumRaw {
    points: i32,
    vegetables: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvenOddRaw {
    even: i32,
    odd: i32,
    vegetable: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FewestMostRaw {
    points: i32,
    vegetable: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherRaw {
    points: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VegetablePointsRaw {
    points: i32,
    vegetable: String,
}

fn vegetable_into_i32(vegetable: &str) -> i32 {
    match vegetable {
        "BadVeggie" => 0,
        "Tomato" => 1,
        "Carrot" => 2,
        "Lettuce" => 3,
        "Cabbage" => 4,
        "Pepper" => 5,
        "Onion" => 6,
        _ => panic!("Invalid vegetable type {:?}", vegetable),
    }
}

fn point_type_into_i32(point_type: &str) -> i32 {
    match point_type {
        "BadPointType" => 0,
        "PointsPerVegetableOne" => 1,
        "PointsPerVegetableTwo" => 2,
        "PointsPerVegetableThree" => 3,
        "SumTwo" => 4,
        "SumThree" => 5,
        "EvenOdd" => 6,
        "Fewest" => 7,
        "Most" => 8,
        "MostTotal" => 9,
        "FewestTotal" => 10,
        "CompleteSet" => 11,
        "AtLeastTwo" => 12,
        "AtLeastThree" => 13,
        "MissingVegetable" => 14,
        _ => panic!("Invalid point type {:?}", point_type),
    }
}


impl From<CardRaw> for Card {
    fn from(raw: CardRaw) -> Self {
        let points_per_vegetable: Option<PointsPerVegetable>;
        let sum: Option<Sum>;
        let even_odd: Option<EvenOdd>;
        let fewest_most: Option<FewestMost>;
        let other: Option<Other>;

        match raw.point_type.as_str() {
            "PointsPerVegetableOne" | "PointsPerVegetableTwo" | "PointsPerVegetableThree" => {
                points_per_vegetable = Some(PointsPerVegetable{
                    points: serde_json::from_value::<Vec<VegetablePointsRaw>>(raw.details).unwrap().into_iter().map(VegetablePoints::from).collect(),
                });
                sum = None; // No need to set these since they're not used
                even_odd = None;
                fewest_most = None;
                other = None;
            }
            "SumTwo" | "SumThree" => {
                points_per_vegetable = None;
                sum = Some(Sum::from(
                    serde_json::from_value::<SumRaw>(raw.details).unwrap(),
                ));
                even_odd = None;
                fewest_most = None;
                other = None;
            }
            "EvenOdd" => {
                points_per_vegetable = None;
                sum = None;
                even_odd = Some(EvenOdd::from(
                    serde_json::from_value::<EvenOddRaw>(raw.details).unwrap(),
                ));
                fewest_most = None;
                other = None;
            }
            "Fewest" | "Most" => {
                points_per_vegetable = None;
                sum = None;
                even_odd = None;
                fewest_most = Some(FewestMost::from(
                    serde_json::from_value::<FewestMostRaw>(raw.details).unwrap(),
                ));
                other = None;
            }
            "MostTotal" | "FewestTotal" | "CompleteSet" | "AtLeastTwo" | "AtLeastThree" | "MissingVegetable" => {
                points_per_vegetable = None;
                sum = None;
                even_odd = None;
                fewest_most = None;
                other = Some(Other::from(
                    serde_json::from_value::<OtherRaw>(raw.details).unwrap(),
                ));
            }
            _ => {
                panic!("Invalid point type");
            }
        }

        Card {
            card_id: raw.card_id,
            vegetable: vegetable_into_i32(&raw.vegetable),
            point_type: point_type_into_i32(&raw.point_type),
            points_per_vegetable: points_per_vegetable,
            sum: sum,
            even_odd: even_odd,
            fewest_most: fewest_most,
            other: other,
        }
    }
}

impl From<PointsPerVegetableRaw> for PointsPerVegetable {
    fn from(raw: PointsPerVegetableRaw) -> Self {
        PointsPerVegetable {
            points: raw.points.into_iter().map(VegetablePoints::from).collect(),
        }
    }
}

impl From<SumRaw> for Sum {
    fn from(raw: SumRaw) -> Self {
        Sum {
            points: raw.points,
            vegetables: raw.vegetables.into_iter()
                .map(|value|vegetable_into_i32(&value))
                .collect(),
        }
    }
}

impl From<EvenOddRaw> for EvenOdd {
    fn from(raw: EvenOddRaw) -> Self {
        EvenOdd {
            even: raw.even,
            odd: raw.odd,
            vegetable: vegetable_into_i32(&raw.vegetable)
        }
    }
}

impl From<FewestMostRaw> for FewestMost {
    fn from(raw: FewestMostRaw) -> Self {
        FewestMost {
            points: raw.points,
            vegetable: vegetable_into_i32(&raw.vegetable)
        }
    }
}

impl From<OtherRaw> for Other {
    fn from(raw: OtherRaw) -> Self {
        Other {
            points: raw.points,
        }
    }
}

impl From<VegetablePointsRaw> for VegetablePoints {
    fn from(raw: VegetablePointsRaw) -> Self {
        VegetablePoints {
            points: raw.points,
            vegetable: vegetable_into_i32(&raw.vegetable)
        }
    }
}

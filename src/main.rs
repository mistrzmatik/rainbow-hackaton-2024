use std::{fs, io::Read};

// use point_salad::{game_client::GameClient, Card};


mod models;

fn load_all_cards(file_name: String) -> Vec<models::Card> {
    let mut file = fs::File::open(file_name).expect("Unable to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Unable to read file");

    let cards: Vec<models::Card> = serde_json::from_str(&json_data)
        .expect("JSON was not well-formatted");

    cards
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vegetables = models::Vegetable::list();
    let cards = load_all_cards("data.json".to_string());

    for card in cards {
        println!("{:?}", card);
    }

    Ok(())
}
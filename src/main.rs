use std::fs;
use std::io::Read;
use point_salad_server::game_client::GameClient;
use point_salad_server::Card;
use point_salad_server::Config;
use point_salad_server::GetRoomRequest;
use point_salad_server::JoinRoomRequest;
use point_salad_server::RoomState;
use point_salad_server::MoveRequest;
use point_salad_server::MoveType;
use tonic::Request;
use crate::strategies::min_max::MinMaxStrategy;
use crate::strategies::random::RandomStrategy;
use crate::strategies::strategy::Strategy;

mod point_salad_server;
mod strategies;
mod models;
mod helpers;
mod points;
mod better_game_state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let cards = load_all_cards("data.json".to_string());

    //for card in cards {
    //    println!("{:?}", card);
    //}

    //return Ok(());
    
    //let url = "http://hackaton-2024.rainbowtours.pl:80";
    let url = "http://[::1]:50051";
    let mut player_name = "rozrewolwerowana konstantynopolitańczykowianeczka".to_string();
    let room_id = "abcd";
    let new = std::env::args().any(|arg| arg == "--new") || std::env::var("NEW").is_ok();
    let number_of_games = 50;

    let mut strategy: Box<dyn Strategy> = if std::env::var("MINMAX").is_ok() {
        player_name += " - MM";
        Box::new(MinMaxStrategy::new(false))
    }
    else if std::env::var("MINMAX_FLIP").is_ok() {
        player_name += " - MMF";
        Box::new(MinMaxStrategy::new(true))
    }
    else { 
        Box::new(RandomStrategy::new()) 
    };
    
    let mut client = GameClient::connect(url).await?;

    if new {
        let request = Request::new(Config {
            room_id: room_id.to_string(),
            number_of_games,
        });

        client.new_room(request).await?;

        println!(
            "Stworzyłem i dołączam do pokoju o Idku: {}. Czekam na innych graczy :)",
            room_id
        );
    }

    let request = Request::new(JoinRoomRequest {
        player_name: player_name.to_string(),
        room_id: room_id.to_string(),
    });

    let mut room_state: RoomState = client.join_room(request).await?.into_inner();

    println!(
        "Dołączyłem do pokoju o Idku: {} :), {:?}",
        room_id, room_state
    );

    while room_state.start_next_game {
        let request = Request::new(GetRoomRequest {
            player_id: room_state.player_id.to_string(),
            room_id: room_id.to_string(),
        });
        let mut game_state = client.get_current_game_state(request).await?.into_inner();

        while !game_state.is_game_over {
            loop {
                let request = if game_state.move_to_make == <MoveType as Into<i32>>::into(MoveType::TakeCards) {
                    let cards = strategy.make_take_cards_move(&game_state);

                    println!(
                        "Biorę karty: {:?}",
                        cards
                    );

                    Request::new(MoveRequest {
                        player_id: room_state.player_id.to_string(),
                        room_id: room_id.to_string(),
                        move_type: MoveType::TakeCards.into(),
                        cards
                    })
                } else {
                    let cards = strategy.make_flip_move(&game_state);

                    if !cards.is_empty() {
                        println!(
                            "Flipuje karty: {:?}",
                            cards
                        );
                    }
                    
                    Request::new(MoveRequest {
                        player_id: room_state.player_id.to_string(),
                        room_id: room_id.to_string(),
                        move_type: MoveType::FlipCard.into(),
                        cards,
                    })
                };

                match client.make_move(request).await {
                    Ok(request) => {
                        game_state = request.into_inner();
                        break;
                    },
                    Err(e) => {
                        println!(
                            "Error z serwera: {:?}",
                            e
                        );
                    }
                }
            }
        }

        let request = Request::new(GetRoomRequest {
            player_id: room_state.player_id.to_string(),
            room_id: room_id.to_string(),
        });
        room_state = client.get_room_state(request).await?.into_inner();
        println!("State: {:?}", room_state);
    }

    Ok(())
}

fn load_all_cards(file_name: String) -> Vec<Card> {
    let mut file = fs::File::open(file_name).expect("Unable to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Unable to read file");

    let cardRaws: Vec<models::CardRaw> = serde_json::from_str(&json_data)
        .expect("JSON was not well-formatted");

    let cards = cardRaws.into_iter().map(Card::from).collect();

    cards
}

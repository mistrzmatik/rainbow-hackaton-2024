use point_salad::game_client::GameClient;

pub mod point_salad {
    tonic::include_proto!("point_salad");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GameClient::connect("http://[::1]:10000").await?;

//     // let request = tonic::Request::new(HelloRequest {
//     //     name: "Tonic".into(),
//     // });

//     // let response = client.say_hello(request).await?;

//     // println!("RESPONSE={:?}", response);

    Ok(())
}
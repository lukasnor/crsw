use std::error::Error;
use json;
use reqwest;
use tokio;
use crsw::ecke;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder().build()?;
    let resp = client
        .get("https://spiele.zeit.de/eckeapi/game/available/regular")
        .send()
        .await?
        .text()
        .await?;
    let info = json::parse(&resp).unwrap();
    let id = info[0]["id"].as_usize().ok_or("Not able to parse id")?;
    let resp = client
        .get(String::from("https://spiele.zeit.de/eckeapi/game/") + &id.to_string())
        .send()
        .await?
        .text()
        .await?;
    let test_game: ecke::Game = serde_json::from_str(&resp)?;
    println!("");
    println!("{}", test_game.latex());
    for question in test_game.questions {
        println!("{}", question);
    }
    Ok(())
}

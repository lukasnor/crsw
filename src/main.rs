use tokio;
use crsw::{Config, Game};
use std::{env, process};

#[tokio::main]
async fn main(){
    let mut args : Vec<String> = env::args().collect();
    args.reverse();// so pop yields the "first" element from now on
    let config = Config::build(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let game = crsw::from_config(config).await.unwrap_or_else(|err| {
        println!("Problem in creating a game: {}", err);
        process::exit(1);
    });
    println!("{}",game.latex());
    process::exit(0);
}

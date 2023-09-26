use crsw::Config;
use std::{env, process};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.reverse(); // so pop yields the "first" element from now on
    let mut config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    config.execute().unwrap_or_else(|err| {
        eprintln!("Problem in module {}\n{}", config.module, err);
        process::exit(1);
    });

    process::exit(0);
}

mod modal;
mod services;

use std::env::args;
use reqwest::Error;
use services::Services;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    let services = Services{ url: String::from("https://pokeapi.co/api/v2") };

    // TODO:
    // 1. List all pokemons - done
    // 2. Details of moves
    // 3. Search pokemon
    // 4. Move details
    // 5. Help

    match args[1].as_str() {
        "list" => {
            let _ = services.get_all_pokemon().await?;
        },
        "move" => {
            let _ = services.get_all_move().await?;
        },
        "pokemon" => {
            let _ = services.get_pokemon(&(args[2].as_str())).await?;
        },
        _ => {
            println!("{} is not a valid command", args[1]);
        }
    }
    Ok(())
}

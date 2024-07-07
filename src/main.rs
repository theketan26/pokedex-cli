mod modal;
mod services;

use std::io::stdin;
use reqwest::Error;
use services::Services;


#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("\nWelcome to the CLI Pokedex!");

    let services = Services{ url: String::from("https://pokeapi.co/api/v2") };

    let mut input = String::new();
    loop {
        println!("1.\tList All Pokemons\n2.\tList All Moves\n3.\tList All Types\n4.\tSearch by name\n0.\tExit\nEnter your choice..");
        stdin().read_line(&mut input).unwrap();

        match input.as_str().trim() {
            "1" => {
                let _ = services.get_all_pokemon().await?;
            },
            "2" => {
                let _ = services.get_all_move().await?;
            },
            "3" => {
                let _ = services.get_all_type().await?;
            },
            "4" => {
                let _ = services.search_pokemon().await?;
            },
            "0" => {
                break;
            },
            _ => {
                println!("{} is not a valid command", input);
            }
        }

        input.clear()
    }

    Ok(())
}

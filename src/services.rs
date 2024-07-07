use std::io::stdin;
use reqwest::{ get, Error };
use serde_json::from_str;
use strsim::jaro;

use crate::modal::{ Pokemon, PokemonList };


pub struct Services {
    pub url: String,
}


impl Services {
    pub async fn get_pokemon(&self, pokemon: &str) -> Result<(), Error> {
        let res = get(format!("{url}/pokemon/{pokemon}", url = self.url)).await;
        
        match res {
            Ok(response) => {
                let res_text: &str = &response.text().await?;

                if res_text == "Not Found" {
                    println!("Could not find the pokemon {}, please check for spelling mistake.", pokemon)
                } else {
                    let res_json: Pokemon = from_str(&res_text).unwrap();
                    println!("{:#?}", res_json);
                }
                
            },
            Err(err) => println!("Faced an error: {}", err)
        }
        Ok(())
    }
    
    pub async fn get_all_type(&self) -> Result<(), Error> {
        let res = get(format!("{url}/type", url = self.url)).await;
        
        match res {
            Ok(response) => {
                let res_text: &str = &response.text().await?;

                let res_json: PokemonList = from_str(&res_text).unwrap();
                println!("{:#?}", res_json);
            },
            Err(err) => println!("Faced an error: {}", err)
        }
        Ok(())
    }
    
    pub async fn get_all_move(&self) -> Result<(), Error> {
        let mut url = format!("{url}/move", url = self.url);
        let mut running = true;

        loop {
            if !running { break };

            let res = get(format!("{url}")).await;
            match res {
                Ok(response) => {
                    let res_text: &str = &response.text().await?;
                    let res_json: PokemonList;
    
                    if res_text == "Not Found" {
                        println!("Could not list pokemon moves.");
                        panic!();
                    } else {
                        res_json = from_str(&res_text).unwrap();
                        println!("{:#?}", res_json);
                    }
                    
                    let mut input = String::new();
                    loop {
                        println!("Enter:\n1: Next moves\n2: Previous moves\n0: To exit");

                        let _ = stdin().read_line(&mut input).unwrap();

                        match input.as_str().trim() {
                            "1" => {
                                let next_url = res_json.next.clone();
                                if next_url.is_none() {
                                    println!("No more moves to display.");
                                    input.clear();
                                    continue;
                                }
                                url = next_url.unwrap();
                                break;
                            }, 
                            "2" => {
                                let prev_url = res_json.previous.clone();
                                if prev_url.is_none() {
                                    println!("No previous moves to display.");
                                    input.clear();
                                    continue;
                                }
                                url = prev_url.unwrap();
                                break;
                            },
                            "0" => {
                                println!("Exiting the Pokedex.");
                                running = false;
                                break;
                            },
                            _ => {
                                println!("Invalid input!")
                            }
                        } 
                    }
                },
                Err(err) => println!("Faced an error: {}", err)
            }
        }
        Ok(())
    }

    pub async fn get_all_pokemon(&self) -> Result<(), Error> {
        let mut url = format!("{url}/pokemon", url = self.url);
        let mut running = true;

        loop {
            if !running { break };

            let res = get(format!("{url}")).await;
            match res {
                Ok(response) => {
                    let res_text: &str = &response.text().await?;
                    let res_json: PokemonList;
    
                    if res_text == "Not Found" {
                        println!("Could not list pokemons.");
                        panic!();
                    } else {
                        res_json = from_str(&res_text).unwrap();
                        println!("{:#?}", res_json);
                    }
                    
                    let mut input = String::new();
                    loop {
                        println!("Enter:\n1: Next pokemons\n2: Previous pokemons\n0: To exit");

                        let _ = stdin().read_line(&mut input).unwrap();

                        match input.as_str().trim() {
                            "1" => {
                                let next_url = res_json.next.clone();
                                if next_url.is_none() {
                                    println!("No more pokemons to display.");
                                    input.clear();
                                    continue;
                                }
                                url = next_url.unwrap();
                                break;
                            }, 
                            "2" => {
                                let prev_url = res_json.previous.clone();
                                if prev_url.is_none() {
                                    println!("No previous pokemons to display.");
                                    input.clear();
                                    continue;
                                }
                                url = prev_url.unwrap();
                                break;
                            },
                            "0" => {
                                println!("Exiting the Pokedex.");
                                running = false;
                                break;
                            },
                            _ => {
                                println!("Invalid input!")
                            }
                        } 
                    }
                },
                Err(err) => println!("Faced an error: {}", err)
            }
        }
        Ok(())
    }

    pub async fn search_pokemon(&self, pokemon: &str) -> Result<(), Error> {
        let res = get(format!("{url}/pokemon/?limit=9999", url = self.url)).await;
        
        match res {
            Ok(response) => {
                let res_text: &str = &response.text().await?;
                let res_json: PokemonList = from_str(res_text).unwrap();

                for pokemon_name in res_json.results.into_iter() {
                    let similarity = jaro(pokemon, &pokemon_name.name);
                    if similarity > 0.8 {
                        println!("{:#?}", pokemon_name);
                    }
                }
            },
            Err(err) => {
                println!("Faced an error: {}", err);
            }
        }

        Ok(())
    }
}
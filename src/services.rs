use reqwest::{ get, Error };
use serde_json::from_str;

use crate::modal::Pokemon;


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
}
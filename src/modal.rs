use serde::{ Serialize, Deserialize };


#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
    pub url: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    pub id: usize,
    pub name: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MoveDamageClass {
    pub id: usize,
    pub name: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    pub id: usize,
    pub name: String,
    pub accuracy: usize,
    pub pp: usize,
    pub priority: usize,
    pub power: usize,
    pub damage_class: MoveDamageClass,
    pub r#type: Type,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: usize,
    pub ability: Name,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonState {
    pub stat: Name,
    pub base_stat: usize,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonMoveVersion {
    pub move_learn_method: Name,
    pub version_group: Name,
    pub level_learned_at: usize,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonMove {
    pub r#move: Name,
    pub version_group_details: Vec<PokemonMoveVersion>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonType {
    pub slot: usize,
    pub r#type: Name,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: usize,
    pub name: String,
    pub height: usize,
    pub weight: usize,
    pub stats: Vec<PokemonState>,
    pub abilities: Vec<PokemonAbility>,
    pub forms: Vec<Name>,
    pub moves: Vec<PokemonMove>,
    pub types: Vec<PokemonType>,
}
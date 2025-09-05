use std::{fs::File, io::{self, Read, Write}};

use crate::{structs::game_engine::GameEngine};



/// Reads settings.json, file and creates and returns **GameEngine** struct from the values. 
pub fn get_engine_from_file(file_path:&str) -> Result<GameEngine, io::Error> {

    let mut file_text: String = String::new();
    File::open(file_path)?.read_to_string(&mut file_text)?;

    let parsed_data: GameEngine = serde_json::from_str(&file_text)?;

    Ok(parsed_data)

}

pub fn create_engine_init_file(game_engine_to_json: &GameEngine, output_path: &str) -> Result<(),io::Error> {

    let json_string = serde_json::to_string(game_engine_to_json)?;

    let mut created_file = File::create(output_path)?;

    created_file.write_all( &json_string.as_bytes() )?;

    Ok(())

}
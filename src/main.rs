use std::{fs::File, io::{self, ErrorKind, Read, Write}};

use crate::{consts::SETTING_FILE_PATH, enums::{game_objects::GameObject, game_status::GameStatus}, structs::{game_engine::GameEngine, object_cordinate::ObjectCordinate}, utils::handle_input::listen_to_input_for};

mod structs{
    pub mod game_engine;
    pub mod object_cordinate;
}

mod enums{
    pub mod directions;
    pub mod game_objects;
    pub mod game_status;
}

mod utils {
    pub mod handle_input;
    pub mod level_utils;
    pub mod rendering;
    pub mod game_logic;
}

#[cfg(test)]
mod tests{
    mod game_logic_tests;
}

mod consts;

fn main() {

    let wait_time_in_sec_after_error: u64 = 10 * 1000;

    let snake_starting_x = 3;
    let snake_starting_y = 3;

    let mut default_snake: Vec<ObjectCordinate> = Vec::new();
    default_snake.push(ObjectCordinate{x: snake_starting_x, y: snake_starting_y, visual_part: GameObject::SnakeHead});


    // Default engine
    let mut engine :GameEngine = GameEngine {
        fps: consts::FPS,
        level_row_size: 10,
        level_column_size: 10,
        max_fruit_count: 10,
        move_after_key_press: true,
        snake: default_snake,
        game_status: GameStatus::Active
    };
    
    //overwrites the default engine, if one is found from "./settings.json"
    match get_engine_from_file() {

        Ok( new_engine)=> {
            
            engine = new_engine;
        }

        Err(err) => {

            match err.kind() {
                
                ErrorKind::NotFound => {
                    println!("{}, not found", SETTING_FILE_PATH);
                    println!("Using dedault engine settings");
                    println!("Trying to create a {}", SETTING_FILE_PATH);
                    
                    let result_of_creating_init = create_engine_init_file(&engine);

                    if result_of_creating_init.is_ok(){
                        println!("Creating init file: OK!")
                    }
                    else{
                        println!("Creating init file: FAILED!")
                    }

                }

                _ => {
                    println!("using default engine, because error: {}", err);
                }
            }
            
            


            println!("Game will start in {} seconds, press any key to start now...", (wait_time_in_sec_after_error / 1000) );
            listen_to_input_for(&wait_time_in_sec_after_error, &true);
        }

    };

    match engine.validate_settings() {
        
        Ok(_return_value) => {},

        Err(err_message) => {
            println!("{}, Program will exit in {} seconds. Press any key to exit now...",err_message, (wait_time_in_sec_after_error / 1000));
            listen_to_input_for(&wait_time_in_sec_after_error, &true);
            return;
        },

    }

    engine.start();
}


/// Reads settings.json, file and creates and returns **GameEngine** struct from the values. 
fn get_engine_from_file() -> Result<GameEngine, io::Error> {

    let mut file_text = String::new();
    File::open(SETTING_FILE_PATH)?.read_to_string(&mut file_text)?;

    let parsed_data: GameEngine = serde_json::from_str(&file_text)?;

    Ok(parsed_data)

}

fn create_engine_init_file(game_engine_to_json: &GameEngine) -> Result<(),io::Error> {

    let json_string = serde_json::to_string(game_engine_to_json)?;

    let mut created_file = File::create(SETTING_FILE_PATH)?;

    created_file.write_all( &json_string.as_bytes() )?;

    Ok(())

}
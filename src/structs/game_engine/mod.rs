use std::{io::{stdout}, time::{Duration, SystemTime}};
use crossterm::event::KeyCode;
use ratatui::{prelude::CrosstermBackend, Terminal};
use serde::{Deserialize, Serialize};

use crate::{ 
    consts::WAIT_TIME_BEFORE_START, enums::{
        directions::Directions, game_objects::GameObject, game_status::GameStatus}, structs::object_cordinate::ObjectCordinate, utils::{game_logic::{check_to_add_fruits, get_next_game_status, place_snake_to_level, wait_before_start}, handle_input::{handle_input, listen_to_input_for}, level_utils::create_level_vec, rendering::{check_if_terminal_window_big_enough, render_level, render_lose_screen, render_paused_screen, render_win_screen}}
};

#[derive(Deserialize, Serialize)]
pub struct GameEngine{

    pub fps: u32,
    pub level_row_size: u32,
    pub level_column_size: u32,
    pub max_fruit_count: u32,
    pub game_status: GameStatus,

    // When listening for inputs, this will decide if after keypress the keypress should be processed immediately
    // or if it will keep listening for the inputs until the "input_wait_time" duration has been waited. 
    pub move_after_key_press: bool,

    // [0] is always "GameObject::SnakeHead", rest are GameObject::SnakeBody
    // When moving inserts new head to [0] and changes the old head to SnakeBody,
    // then removes the last part of the snake, unless fruit is eaten.
    pub snake: Vec<ObjectCordinate>, 
}

impl GameEngine {

    pub fn start(&mut self){
        let input_wait_time: u64 = (1000 / &self.fps) as u64; // in ms
        let mut current_direction: Directions = Directions::Up;

        //keeps track of the position of fruits
        let mut fruits_vec: Vec<ObjectCordinate> = Vec::new();

        // vector contains column_size amount of rows that are vectors.
        let mut level_vec = create_level_vec(&self.level_row_size, &self.level_column_size); 
        place_snake_to_level(&mut level_vec, &mut self.snake);
        check_to_add_fruits(&mut level_vec, &mut fruits_vec, &self.max_fruit_count);


        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();
        

        let got_error_in_wait = wait_before_start(&mut level_vec, &mut terminal , WAIT_TIME_BEFORE_START);
        
        if got_error_in_wait.is_err() {

            println!("terminal window isn't big enough");
            println!("Game starts once terminal windows is resized");

            loop {
                if check_if_terminal_window_big_enough(&mut terminal, &level_vec).is_ok() {
                    break;
                }
            }
        }


        //main loop
        loop {
            
            //waits for inputs for "input_wait_time" in ms, and handles potenttial inputs 
            let given_input: KeyCode = listen_to_input_for(&input_wait_time, &self.move_after_key_press);
            handle_input(&mut current_direction, &given_input, &mut self.game_status);

            if self.game_status == GameStatus::Paused
            {
                // render pause state
                if render_paused_screen(&mut terminal, &level_vec).is_err() {
                    println!("terminal not big enough");
                    break;
                }
                
                continue;
            }
            
            else if self.game_status == GameStatus::Active {
                self.game_status = get_next_game_status(&mut level_vec, &mut self.snake, &current_direction, &mut fruits_vec);
                
                if render_level(&mut terminal, &level_vec).is_err() {
                    println!("terminal not big enough");
                    break;
                }
                
                check_to_add_fruits(&mut level_vec, &mut fruits_vec, &self.max_fruit_count);
            }
    
            else if self.game_status == GameStatus::Lost {
                let current_time = SystemTime::now();
                while current_time.elapsed().unwrap() < Duration::from_millis(500)  {
                    render_lose_screen(&mut terminal);
                }
                return;
            }
            
            else if self.game_status == GameStatus::Won {
                let current_time = SystemTime::now();
                while current_time.elapsed().unwrap() < Duration::from_millis(500)  {
                    render_win_screen(&mut terminal);
                }
                return;
            }

        }

        ratatui::restore();
        
    }
    
    
    pub fn validate_settings(&mut self) -> Result<(), &str> {

        if self.fps == 0 {
            return Err("fps can't be 0");
        }

        for i in 0..self.snake.len() {

            let current_snake_part = &self.snake[i];

            if current_snake_part.x >  self.level_row_size {
                return Err("part of snake outside of level");
            }

            if current_snake_part.y >  self.level_column_size {
                return Err("part of snake outside of level");
            }

        }

        if self.snake[0].visual_part !=  GameObject::SnakeHead {
            return Err("first value in snake vec must be 'SnakeHead'")
        }

        Ok(())
    }


}


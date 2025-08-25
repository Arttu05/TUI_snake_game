use std::time::{Duration, SystemTime};
use crate::consts::PAUSE_KEY;
use crate::{consts, enums::game_status::GameStatus};
use crate::enums::directions::Directions;
use crossterm::event::{
    poll, read, Event, KeyCode 
};


pub fn listen_to_input_for(wait_time : &u64, return_after_input: &bool) -> KeyCode {

    let mut return_value = KeyCode::Null;
    let time_before_input = SystemTime::now();

    let wait_time_as_duration = Duration::from_millis(*wait_time);

    let mut time_elapsed = time_before_input.elapsed().unwrap(); 

    // when "return_after_input" is true, or when pause is pressed. Makes the first loop/keypress during this function invalid.
    // This is because, when pressing a key it will be returned and the next game stage is calculated and so on...,
    // but when this function is called again, the previous key that was pressed, triggers again and exits this function immediately.
    // TLDR: prevents key's from triggering twice.
    let mut loop_count:u32 = 0;

    while time_elapsed <= wait_time_as_duration    {
        
        if poll(wait_time_as_duration - time_elapsed).unwrap(){
            match read().unwrap() {
                Event::Key(event) => {

                    if *return_after_input && loop_count >= 1 {
                        return  event.code;
                    }

                    if loop_count >= 5 && event.code == PAUSE_KEY {
                        return  event.code;
                    } 

                    return_value = event.code
                },
                _ => {}
            }
        };

        loop_count += 1;

        time_elapsed = time_before_input.elapsed().unwrap();

    }


    return_value

}

pub fn handle_input(current_direction: &mut Directions, given_input : &KeyCode, current_status: &mut GameStatus  ) {

    match *given_input {
        consts::DOWN_KEY => {

            let new_direction = Directions::Down;
            
            validate_new_direction(current_direction, new_direction);
        }
        consts::LEFT_KEY => {

            let new_direction = Directions::Left;

            validate_new_direction(current_direction, new_direction);
        }
        consts::UP_KEY => {

            let new_direction = Directions::Up;

            validate_new_direction(current_direction, new_direction);
        }
        consts::RIGHT_KEY => {

            let new_direction = Directions::Right;

            validate_new_direction(current_direction, new_direction);
        }

        consts::PAUSE_KEY => {

            if *current_status == GameStatus::Paused {
                *current_status = GameStatus::Active; 
            }
            else {
                *current_status = GameStatus::Paused; 
            }

        }
        
        _ => {}
    }
    
}

fn validate_new_direction(current_direction: &mut Directions, new_direction: Directions ) -> bool{

    if *current_direction != new_direction && opposite_direction_of(&current_direction) != new_direction {

        *current_direction = new_direction;

        return true;
    }    
    else{
        return false;
    }

}

fn opposite_direction_of(direction: &Directions ) -> Directions{
    
    match *direction {
        Directions::Down => {
            return  Directions::Up;
        }
        Directions::Left => {
            return  Directions::Right;
        }
        Directions::Up => {
            return  Directions::Down;
        }
        Directions::Right => {
            return  Directions::Left;
        }
    }

}

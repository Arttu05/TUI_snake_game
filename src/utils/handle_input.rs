use std::time::{Duration, SystemTime};
use crate::consts::PAUSE_KEY;
use crate::{consts, enums::game_status::GameStatus};
use crate::enums::directions::Directions;
use crossterm::event::{
    poll, read, Event, KeyCode 
};

///Wait time in milliseconds
pub fn listen_to_input_for(wait_time : &u64, return_after_input: &bool) -> KeyCode {

    let mut return_value = KeyCode::Null;
    let time_before_input = SystemTime::now();

    let wait_time_as_duration = Duration::from_millis(*wait_time);

    let mut time_elapsed = time_before_input.elapsed().unwrap(); 

    while time_elapsed <= wait_time_as_duration    {
        
        if poll(wait_time_as_duration - time_elapsed).unwrap(){
            
            let current_event = read().unwrap();
            
            if current_event.is_key_press() == false {
                continue;
            }

            match current_event {                

                Event::Key(event) => {

                    if *return_after_input {
                        return  event.code;
                    }

                    if event.code == PAUSE_KEY {
                        return  event.code;
                    } 

                    return_value = event.code
                },
                _ => {}
            }
        };

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

use std::io::{Error, Stdout};
use std::time::{Duration, SystemTime};

use crate::consts::LOSE_ON_COLLISION;
use crate::enums::directions::Directions;
use crate::utils::rendering::render_level;
use rand::{rng, Rng};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use crate::enums::game_objects::GameObject;
use crate::enums::game_status::GameStatus;
use crate::structs::object_cordinate::ObjectCordinate;
use crate::utils::level_utils::{get_count_of, get_game_object_from, get_locations_of};

/// Validates the next move and moves the snake. 
/// 
/// - Returns **GameStatus**
pub fn get_next_game_status(level_vec: &mut Vec<Vec<GameObject>>, snake: &mut Vec<ObjectCordinate> , move_direction: &Directions, fruits_vec: &mut Vec<ObjectCordinate>) -> GameStatus{


    if check_win(&level_vec) {
        return GameStatus::Won;
    }


    let (new_x, new_y, ate_fruit) = match get_next_position(level_vec, &snake, move_direction, fruits_vec){
        Ok((x,y, ate_fruit)) => {
            (x,y, ate_fruit)
        },
        Err(_string) => {
            return GameStatus::Lost;
        }
    };

    move_snake(level_vec, snake, &new_x, &new_y, &ate_fruit);
    place_snake_to_level(level_vec, &snake);
    GameStatus::Active
}

///Checks how many fruits should be added and then adds the fruits randomly. 
/// This function also makes sure there are enough empty spaces to add new fruits 
pub fn check_to_add_fruits(level_vec: &mut Vec<Vec<GameObject>>, fruits_vec: &mut Vec<ObjectCordinate>, max_fruit_count: &u32 ){
    let mut available_spaces = get_locations_of(level_vec, GameObject::EmptySpace);
    
    while fruits_vec.len() as u32 <= (*max_fruit_count - 1) {

        if available_spaces.len() == 0 {

            break;

        }

        add_fruit_to_random(level_vec, fruits_vec, &mut available_spaces);

    }

}

///renders the game screen for **wait_time**. doesn't listen for input during this.
pub fn wait_before_start(level_vec: &mut Vec<Vec<GameObject>>, ter: &mut Terminal<CrosstermBackend<Stdout>>, wait_time: Duration) -> Result<(), Error>{

    let time_at_start = SystemTime::now();

    while time_at_start.elapsed().unwrap() < wait_time {
        
        render_level(ter, level_vec)?;

    }

    Ok(())

}


//########################################
// PRIVATE FUNTIONS
//########################################


fn add_fruit_to_random(level_vec: &mut Vec<Vec<GameObject>>, fruits_vec: &mut Vec<ObjectCordinate>, available_spaces: &mut Vec<ObjectCordinate>){
    
    let mut random_space_index = 0;

    if available_spaces.len() != 0 {

        random_space_index = rng().random_range(0..(available_spaces.len()));
    }


    let random_space = &available_spaces[random_space_index];

    level_vec[random_space.y as usize][random_space.x as usize] = GameObject::Fruit;
    fruits_vec.push(ObjectCordinate { x: (random_space.x), y: (random_space.y), visual_part: (GameObject::Fruit) });

    available_spaces.remove(random_space_index);
}

fn check_win(level_vec: &Vec<Vec<GameObject>>) -> bool{


    let count_of_empty_spaces = get_count_of(level_vec, &GameObject::EmptySpace);
    let count_of_fruits = get_count_of(level_vec, &GameObject::Fruit);

    if count_of_empty_spaces == 0 && count_of_fruits == 0 {

        return  true;

    }

    false

}

fn move_snake(level_vec: &mut Vec<Vec<GameObject>>, snake: &mut Vec<ObjectCordinate>, new_x: &u32, new_y: &u32, ate_fruit: &bool ){

    remove_snake_from_level(level_vec, &snake);

    let mut only_head = false;

    let snake_len = snake.len();

    if snake_len == 1 {
        only_head= true;
    }

    if *ate_fruit == false {
        snake.remove(snake_len - 1);
    }

    snake.insert(0, ObjectCordinate { x: (*new_x), y: (*new_y), visual_part: (GameObject::SnakeHead) });

    if only_head == false || *ate_fruit == true {

        snake[1].visual_part = GameObject::SnakeBody

    }
    
    
}

/// loops through the snake Vec and then uses the snake data to remove it from level
fn remove_snake_from_level(level_vec: &mut Vec<Vec<GameObject>>, snake: &Vec<ObjectCordinate>){
    
    for i in 0..snake.len() {
        let y =  snake[i].y;
        let x = snake[i].x;

        level_vec[y as usize][x as usize] = GameObject::EmptySpace;
    }
    
}

pub fn place_snake_to_level(level_vec: &mut Vec<Vec<GameObject>>, snake: &Vec<ObjectCordinate> ){

    for i in 0..snake.len() {
        let y =  snake[i].y;
        let x = snake[i].x;
        let snake_part_object = snake[i].visual_part.clone();

        level_vec[y as usize][x as usize] = snake_part_object;
    }

}

///gets snake's next position and collision.
/// 
/// **OK** returns values are **X**, **Y** and **ate_fruit**
/// 
/// **Err** is returned when next position collides with **LOSE_ON_COLLISION** GameObject
fn get_next_position(game_vec: &mut Vec<Vec<GameObject>>, snake: &Vec<ObjectCordinate> , move_direction: &Directions, fruits_vec: &mut Vec<ObjectCordinate>) -> Result<(u32, u32, bool), String> {

    let next_x: u32;
    let next_y: u32;
    let mut ate_fruit = false;
    
    match move_direction {
        
        Directions::Up => {
            next_y = snake[0].y + 1;
            next_x = snake[0].x;
        }
        Directions::Down => {
            next_y = snake[0].y - 1;
            next_x = snake[0].x;
        }
        Directions::Left => {
            next_x = snake[0].x - 1;
            next_y = snake[0].y;
        }
        Directions::Right => {
            next_x = snake[0].x + 1;
            next_y = snake[0].y;
        }

    }

    let game_object_collision = get_game_object_from(&game_vec, &next_x, &next_y);

    if LOSE_ON_COLLISION.contains(&game_object_collision) {
        return Err(String::from("collision with LOSE_ON_COLLISION game_object"));
    }
    else if game_object_collision == GameObject::Fruit {
        ate_fruit = true;

        let mut index_to_remove = 0;

        for fruit_in_vec in fruits_vec.iter() {

            if(*fruit_in_vec == ObjectCordinate{x: next_x, y: next_y, visual_part: GameObject::Fruit }){
                
                break;
            }

            index_to_remove += 1;

        } 

        fruits_vec.remove(index_to_remove);
    }

    Ok((next_x, next_y, ate_fruit))

}


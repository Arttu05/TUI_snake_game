use crate::{enums::{directions::Directions, game_objects::GameObject, game_status::GameStatus}, structs::object_cordinate::ObjectCordinate, utils::{game_logic::{check_to_add_fruits, get_next_game_status}, level_utils::{create_level_vec, get_count_of}}};


#[test]
fn create_level_test(){

    let column_size = 5;
    let row_size = 3;

    let level_vec = create_level_vec(&row_size, &column_size);

    //function adds walls to the edges
    let expected_column_size = column_size + 2;
    let expected_row_size = row_size + 2;

    assert_eq!(level_vec.len(), expected_column_size as usize);
    assert_eq!(level_vec[0].len(), expected_row_size as usize);

}

#[test]
fn add_fruits_to_level(){

    let column_size = 5;
    let row_size = 3;
    let max_fruit_count = 3;
    let mut level_vec = create_level_vec(&row_size, &column_size);
    let mut fruits_vec: Vec<ObjectCordinate> = Vec::new();
    
    check_to_add_fruits(&mut level_vec, &mut fruits_vec, &max_fruit_count);
    
    let fruit_count_in_level = get_count_of(&level_vec, &GameObject::Fruit);
    
    assert_eq!(fruit_count_in_level, max_fruit_count);
    
}

#[test]
fn lose_on_collision_with_wall(){
    
    let direction_to_move = Directions::Down;
    let column_size = 3;
    let row_size = 3;
    let mut level_vec = create_level_vec(&row_size, &column_size);
    let mut snake: Vec<ObjectCordinate> = Vec::new();
    let mut fruits_vec: Vec<ObjectCordinate> = Vec::new();
    snake.push(ObjectCordinate { x: 1, y: 1, visual_part: GameObject::SnakeHead }); 


    let next_game_state: GameStatus = get_next_game_status(&mut level_vec, &mut snake, &direction_to_move, &mut fruits_vec);

    assert_eq!(next_game_state, GameStatus::Lost);

}

#[test]
fn normal_move(){
    
    let direction_to_move = Directions::Up;
    let column_size = 3;
    let row_size = 3;
    let mut level_vec = create_level_vec(&row_size, &column_size);
    let mut snake: Vec<ObjectCordinate> = Vec::new();
    let mut fruits_vec: Vec<ObjectCordinate> = Vec::new();
    snake.push(ObjectCordinate { x: 1, y: 1, visual_part: GameObject::SnakeHead }); 


    let next_game_state: GameStatus = get_next_game_status(&mut level_vec, &mut snake, &direction_to_move, &mut fruits_vec);

    assert_eq!(next_game_state, GameStatus::Active);

}

#[test]
fn snake_grows_after_eating(){
    
    let direction_to_move = Directions::Up;
    let column_size = 3;
    let row_size = 3;
    let fruit_x: u32 = 1;
    let fruit_y:u32 = 2;
    let mut level_vec = create_level_vec(&row_size, &column_size);
    let mut snake: Vec<ObjectCordinate> = Vec::new();
    let mut fruits_vec: Vec<ObjectCordinate> = Vec::new();
    snake.push(ObjectCordinate { x: 1, y: 1, visual_part: GameObject::SnakeHead }); 
    fruits_vec.push(ObjectCordinate { x: fruit_x, y: fruit_y, visual_part: GameObject::Fruit });

    level_vec[fruit_y as usize][fruit_x as usize] = GameObject::Fruit;

    let snake_len_before = snake.len();

    let next_game_state: GameStatus = get_next_game_status(&mut level_vec, &mut snake, &direction_to_move, &mut fruits_vec);

    if next_game_state != GameStatus::Active {
        panic!("next_game_state wasn't GameStatus::Active")
    }

    let snake_len_after = snake.len();

    assert_ne!(snake_len_before, snake_len_after, "snake size before {}, after {}", snake_len_before, snake_len_after);

}
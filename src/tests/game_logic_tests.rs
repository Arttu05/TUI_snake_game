use crate::{enums::game_objects::GameObject, structs::object_cordinate::ObjectCordinate, utils::{game_logic::check_to_add_fruits, level_utils::{create_level_vec, get_count_of}}};


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
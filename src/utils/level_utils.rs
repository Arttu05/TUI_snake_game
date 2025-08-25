

use crate::{enums::game_objects::GameObject, structs::object_cordinate::ObjectCordinate};

pub fn create_level_vec(row_size: &u32, column_size: &u32 ) -> Vec<Vec<GameObject>>{

    let mut  created_level: Vec<Vec<GameObject>> = Vec::new();

    for _i in 0u32..(*column_size + 2) {

        let row_vec:Vec<GameObject> = vec![GameObject::EmptySpace; (*row_size + 2) as usize];

        created_level.push(row_vec);

    };

    place_walls_to_corners(&mut created_level);

    /* place_snake_to_middle(&mut created_level); */

    created_level

}

pub fn get_game_object_from(level_vec: &Vec<Vec<GameObject>>, x: &u32, y:&u32) -> GameObject{

    let return_value = &level_vec[*y as usize][*x as usize];

    return_value.clone()

}


/// returns how many of **game_object_to_count** exists in **level_vec**
pub fn get_count_of(level_vec: &Vec<Vec<GameObject>>, game_object_to_count: &GameObject) -> u32{

    let mut count = 0;

    for y in 0..level_vec.len(){

        for x in 0..level_vec[0].len(){

            let current_obj = &level_vec[y][x];

            if *current_obj == *game_object_to_count {
                count += 1;
            }

        }

    }

    count

}


pub fn get_locations_of(level_vec: &Vec<Vec<GameObject>>, game_objects_to_find: GameObject) -> Vec<ObjectCordinate> {

    let mut return_value = Vec::new();

    for y in 0..level_vec.len(){

        for x in 0..level_vec[0].len(){

            let current_obj = &level_vec[y][x];

            if game_objects_to_find == *current_obj {

                return_value.push(ObjectCordinate { x: x as u32, y: y as u32, visual_part: current_obj.clone() });

            }


        }

    }

    return_value

}



/* pub fn place_snake_to(vec_level: &mut Vec<Vec<GameObject>>, snake: &Vec<ObjectCordinate>){

    for i in 0..snake.len() {

        let current_snake_part = &snake[i];

        vec_level[current_snake_part.y as usize + 1][current_snake_part.y as usize + 1] = GameObject::SnakeHead;
    
    }

} */


//########################################
// PRIVATE FUNTIONS
//########################################

fn place_walls_to_corners(level_vec: &mut Vec<Vec<GameObject>>){

    for row_index in 0..level_vec.len() {

        let first_row_index = 0;
        let last_row_index = level_vec.len() - 1;

        if row_index == first_row_index || row_index == last_row_index {

            for column_index in 0..level_vec[row_index].len(){

                level_vec[row_index][column_index] = GameObject::Wall;

            }
        }

        else{
            for column_index in 0..level_vec[row_index].len(){

                let first_column_index = 0;
                let last_column_index = level_vec[row_index].len() - 1;

                if column_index == first_column_index || column_index == last_column_index {
                    level_vec[row_index][column_index] = GameObject::Wall;
                }


            }
        }
    }
}


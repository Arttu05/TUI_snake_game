use crate::enums::game_objects::GameObject;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Deserialize, Serialize)]
pub struct ObjectCordinate {

    pub x: u32,
    pub y: u32,
    pub visual_part: GameObject

}
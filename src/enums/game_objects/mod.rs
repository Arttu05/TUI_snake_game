use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(PartialEq, Deserialize, Serialize)]
pub enum GameObject{
    SnakeHead,
    SnakeBody,
    SnakeTail,
    Wall,
    Fruit,
    EmptySpace,
}
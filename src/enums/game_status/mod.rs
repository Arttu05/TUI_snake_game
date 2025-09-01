use serde::{Deserialize, Serialize}; 

#[derive(Deserialize,PartialEq, Serialize, Debug)]
pub enum GameStatus{
    Active,
    Won,
    Lost,
    Paused
}
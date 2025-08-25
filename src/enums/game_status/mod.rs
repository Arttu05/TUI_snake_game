use serde::{Deserialize, Serialize}; 

#[derive(Deserialize,PartialEq, Serialize)]
pub enum GameStatus{
    Active,
    Won,
    Lost,
    Paused
}
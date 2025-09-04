use std::time::Duration;

use crossterm::event::KeyCode;

use crate::enums::game_objects::GameObject;

pub const UP_KEY:KeyCode = KeyCode::Char('w');
pub const RIGHT_KEY:KeyCode = KeyCode::Char('d');
pub const DOWN_KEY:KeyCode = KeyCode::Char('s');
pub const LEFT_KEY:KeyCode = KeyCode::Char('a');
pub const PAUSE_KEY: KeyCode = KeyCode::Esc;


pub const FPS: u32 = 2; 

pub const LOSE_ON_COLLISION: [GameObject; 3] = [GameObject::SnakeBody, GameObject::SnakeTail , GameObject::Wall];

pub const GAME_OVER_TEXT: &str = "GAME OVER";
pub const WIN_TEXT: &str = "WIN!";

pub const WAIT_TIME_BEFORE_START: Duration = Duration::from_millis(300);

pub const SPACE_SIZE_X: u32 = 2;
pub const SPACE_SIZE_Y: u32 = 1;

pub const SETTING_FILE_PATH: &str = "./settings.json";
pub const WAIT_TIME_AFTER_LOSE_OR_WIN: u64 = 10000;
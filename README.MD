# TUI_snake_game

[![Tests](https://github.com/Arttu05/TUI_snake_game/actions/workflows/test_and_build.yml/badge.svg)](https://github.com/Arttu05/TUI_snake_game/actions/workflows/test_and_build.yml)

TUI(terminal user interface) snake game made with rust. Uses ratatui and crossterm to render game to terminal/console.  

## How to play

Compile the project with following commnad: ```cargo build --realease```, while in project's root folder.

Currently controls are hardcoded in ```consts.rs``` and are the following:

- W:     Up
- A:     Left
- S:     Down
- D:     Right
- ESC:   Pause

## Game settings

Game settings are stored in ```./settings.json```. If this file doens't exist the program will create it with default settings to the working dir.

If values are incorrect in ```./settings.json```, the program will use default settings and give warning/notice that it wasn't able to use values in ```./settings.json```.

Datatypes of each setting:

- fps: u32 = how fast the game plays. Waits ```1000 / fps``` for inputs until rendering the next state.

- level_row_size: u32 = Defines how wide the level is

- level_column_size: u32 = Defines how tall the level is

- max_fruit_count: u32 = Defines how many fruits can be on level at once

- game_status: GameStatus (Enum) = Defines the gamestate won, lost, active or paused, should not be changed

- move_after_key_press: bool = While true, after pressing a key immediately handles the input. While false, waits the entire fps time and returns latest input.

- snake: Vec of ObjectCordinates (Struct) = Contains the snake in a list. each value in list is a part of the snake. First value must be ```SnakeHead```. 


## TODO

- Add more tests
- Better win and lose screen

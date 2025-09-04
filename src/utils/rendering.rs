use std::io::{Error, ErrorKind, Stdout};

use ratatui::{layout::{Alignment, Rect}, prelude::CrosstermBackend, style::{Color, Style, Stylize}, widgets::Paragraph, Frame, Terminal};

use crate::{consts::{GAME_OVER_TEXT, SPACE_SIZE_X, SPACE_SIZE_Y, WAIT_TIME_AFTER_LOSE_OR_WIN, WIN_TEXT}, enums::game_objects::GameObject};

pub fn render_level(ter:&mut Terminal<CrosstermBackend<Stdout>>, level_vec: &Vec<Vec<GameObject>>) -> Result<(), Error>{
    
    check_if_terminal_window_big_enough(ter, level_vec)?;

    ter.draw(|frame| {
        
        render_level_from_vec(frame, level_vec);
        
        
    })?;
    
    Ok(())
}

pub fn render_paused_screen(ter:&mut Terminal<CrosstermBackend<Stdout>>, level_vec: &Vec<Vec<GameObject>>) -> Result<(), Error>{

    check_if_terminal_window_big_enough(ter, level_vec)?;

    ter.draw(|frame| {
        
        render_level_from_vec(frame, level_vec);
        let game_center_y =level_vec.len() / 2;

        let paused_text_widget = Paragraph::new("PAUSED")
            .fg(Color::Black)
            .bg(Color::White)
            .alignment(Alignment::Center);

        let pause_widget_width:u16 = level_vec[0].len() as u16 * SPACE_SIZE_X as u16 ;
        let paused_widget_area = Rect::new(0, game_center_y as u16, pause_widget_width, 1);

        frame.render_widget(paused_text_widget, paused_widget_area);

    })?;

    Ok(())

}

pub fn render_lose_screen(ter:&mut Terminal<CrosstermBackend<Stdout>>){

    ter.draw(|frame| {

        render_text_before_exit(frame, GAME_OVER_TEXT);

    }).unwrap();

}

pub fn render_win_screen(ter:&mut Terminal<CrosstermBackend<Stdout>>){

    ter.draw(|frame| {

        render_text_before_exit(frame, WIN_TEXT);

    }).unwrap();

}

pub fn check_if_terminal_window_big_enough(ter:&mut Terminal<CrosstermBackend<Stdout>>, level_vec: &Vec<Vec<GameObject>>) -> Result<(), Error>{

    let max_size = &ter.size()?;
    
    let level_height: u16 = SPACE_SIZE_Y as u16 * level_vec.len() as u16;
    let level_width = SPACE_SIZE_X as u16 * level_vec[0].len() as u16;

    if level_height > max_size.height || level_width > max_size.width {
        return Err(Error::from(ErrorKind::Other));
    }

    Ok(())

}

fn render_text_before_exit(frame: &mut Frame, text_to_render: &str){

    let exit_text = format!("Exit in {} seconds, Press any key to exit now...", (WAIT_TIME_AFTER_LOSE_OR_WIN / 1000));

    let size =frame.area();

    let text_paragraph = Paragraph::new(text_to_render).alignment(Alignment::Center);
    let text_paragraph_area = Rect::new(0, 0, size.width, 1);

    let exit_text = Paragraph::new(exit_text).alignment(Alignment::Center);
    let exit_text_area = Rect::new(
        0, 
        text_paragraph_area.y + 1, 
        size.width, 
        1);

    frame.render_widget(text_paragraph, text_paragraph_area);
    frame.render_widget(exit_text, exit_text_area);

}

fn render_level_from_vec(frame: &mut Frame, level_vec: &Vec<Vec<GameObject>>) {

    for row_index in 0..level_vec.len() {

        // renders from top to bottom in terminal, 
        // so the first rendered row should be the last 
        // and the second row should be second last 
        // and so on...
        let row_index_in_reverse = (level_vec.len() - 1) - row_index; 

        for column_index in 0..level_vec[row_index].len(){
            let space_x = SPACE_SIZE_X * (column_index as u32);
            let space_y = SPACE_SIZE_Y * (row_index as u32);
            
            let visual_char = get_visual_char(&level_vec[row_index_in_reverse][column_index]);
            let char_color = get_color_of_char(&level_vec[row_index_in_reverse][column_index]);
            let background_color = get_background_color(&level_vec[row_index_in_reverse][column_index]);

            let new_space_area = Rect::new(space_x as u16, space_y as u16 ,SPACE_SIZE_X as u16, SPACE_SIZE_Y as u16);
            let new_space_content = Paragraph::new(visual_char)
                .style(Style::default()
                .fg(char_color)
                .bg(background_color)).alignment(Alignment::Center);



            frame.render_widget(new_space_content, new_space_area);

        }
    }

    /* //checks whichs one is bigger and makes the bigger value same size as the smaller one,
    //so the each space is 1:1
    match space_width > space_height {
        true => space_width = space_height,
        false => space_height = space_width
    } */

}

fn get_visual_char(game_obj: &GameObject) -> &str{

    match game_obj {

        GameObject::SnakeBody => return "■",
        GameObject::SnakeTail => return "■",
        GameObject::SnakeHead => return  "●",
        GameObject::Wall => return "■",
        GameObject::Fruit => return "🍉",
        GameObject::EmptySpace => return "□"

    }

}

fn get_color_of_char(game_obj: &GameObject) -> Color{

    match game_obj{

        GameObject::SnakeBody | GameObject::SnakeTail | GameObject::SnakeHead => return Color::Green,
        _ => return  Color::White
    }

}

fn get_background_color(game_obj: &GameObject) -> Color{

    match game_obj{

        GameObject::Wall => return Color::White, 
        _ => return  Color::Black
    }

}
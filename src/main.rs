mod snake_mod;
mod position_on_map;
mod food;
mod items;

use snake_mod::Snake;
use position_on_map::PositionOnMap;
use food::Food;

use ggez;
use ggez::{event, graphics, nalgebra as na, input};
use std::time;
use std::collections::LinkedList;
use std::path;

const MAP_SIZE_X: f32 = 400.0;
const MAP_SIZE_Y: f32 = 400.0;
const CELL_SIZE: i32 = 40;
const START_POS_X: i32 = 40;
const START_POS_Y: i32 = 40;
const TEXT_COLOR: [f32; 4] = [1.0, 0.1, 0.1, 1.0];
const ROUND_TIME: u64 = 500;      //ms


pub struct MainState{
    last_update: time::Instant,
    last_meal: i32,
    snake: Snake,
    alive: bool,
    food_cell: Option<Food>,
    points: i32,
    menu: bool,
    round_time: u64,         
    text_location: f32,
}

impl MainState{
    fn new(pos_x: i32, pos_y: i32) -> ggez::GameResult<MainState>{
        let mut segm = LinkedList::new();
        segm.push_front(PositionOnMap{pos_x: pos_x, pos_y: pos_y});
        segm.push_front(PositionOnMap{pos_x: pos_x + CELL_SIZE, pos_y: pos_y});
        segm.push_front(PositionOnMap{pos_x: pos_x + CELL_SIZE * 2, pos_y: pos_y});
        segm.push_front(PositionOnMap{pos_x: pos_x + CELL_SIZE * 3, pos_y: pos_y});
        segm.push_front(PositionOnMap{pos_x: pos_x + CELL_SIZE * 4, pos_y: pos_y});
        let sn = Snake::new_snake(segm)?;
        let mut f = Food::new_random();
        while sn.collide_with_food(&f){
            f = Food::new_random();
        }
        let s = MainState{
            last_update: time::Instant::now(),
            last_meal: 0,
            snake: sn,
            alive: true,
            food_cell: Some(f),
            points: 0,
            menu: true,
            round_time: ROUND_TIME,
            text_location: CELL_SIZE as f32,
        };
        Ok(s)
    }

    fn draw_grid(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult{
        let mut i = 0.0; 
        while i <= MAP_SIZE_Y{
            let (start, end) = (na::Point2::new(0.0, i), na::Point2::new(MAP_SIZE_X, i));
            let l = graphics::Mesh::new_line(ctx, &[start, end], 1.0, [0.0, 0.0, 0.0, 0.3].into())?;
            graphics::draw(ctx, &l, (na::Point2::new(0.0, 0.0), ))?;
            i += CELL_SIZE as f32;
        }
        i = 0.0;
        while i <= MAP_SIZE_X{
            let (start, end) = (na::Point2::new(i, 0.0), na::Point2::new(i, MAP_SIZE_X));
            let l = graphics::Mesh::new_line(ctx, &[start, end], 1.0, [0.0, 0.0, 0.0, 0.3].into())?;
            graphics::draw(ctx, &l, (na::Point2::new(0.0, 0.0), ))?;
            i += CELL_SIZE as f32;
        }
        Ok(())
    }

    fn eat_food(&mut self){
        self.points += 1;
        self.last_meal = 0;
        if self.round_time >= 400{
            self.round_time -= 20;
        }
        else if self.round_time >= 350{
            self.round_time -= 12;
        }
        else if self.round_time >= 180{
            self.round_time -= 8;
        }

        let mut new_food: Food = Food::new_random();
        while self.snake.collide_with_food(&new_food){
            new_food = Food::new_random();
        }
        self.food_cell = Some(new_food);
    }

}

impl event::EventHandler for MainState{
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult{

        if self.menu{
            if input::keyboard::is_key_pressed(ctx, event::KeyCode::Return){
                self.menu = false;
            }
        }

        // If snake has collided with something, its color changes and its position is no longer being updated
        else if !self.alive{
            self.snake.col = [1.0, 0.0, 0.0, 1.0].into();
            // After pressing Space we create new MainState and overwrite current state with the new one
            if input::keyboard::is_key_pressed(ctx, event::KeyCode::Space){
                let new_state = MainState::new(START_POS_X, START_POS_Y)?;
                *self = new_state;
                return Ok(());
            }
        }

        // Checking if enough time has passed since the last update
        else if time::Instant::now() - self.last_update >= time::Duration::from_millis(self.round_time){
            self.snake.direction = self.snake.new_direction.clone();
            let head = self.snake.get_head().unwrap();

            let new_x: i32 = match self.snake.direction.as_str(){
                "LEFT" => head.pos_x - CELL_SIZE,
                "RIGHT" => head.pos_x + CELL_SIZE,
                _ => head.pos_x,
            };

            let new_y: i32 = match self.snake.direction.as_str(){
                "UP" => head.pos_y - CELL_SIZE,
                "DOWN" => head.pos_y + CELL_SIZE,
                _ => head.pos_y,
            };

            // Checking if the snake has moved
            if new_x != head.pos_x || new_y != head.pos_y{
                let new_pos = PositionOnMap{pos_x: new_x, pos_y: new_y}; 
                self.snake.move_head(new_pos);

                // If snake has eaten some food
                if self.snake.get_head().unwrap() == self.food_cell.as_ref().unwrap().get_pos(){
                    self.eat_food();
                }
                else{
                    self.snake.move_tail();
                    self.last_meal += 1;
                }

                // Checking if the snake has collided
                if self.snake.collide(){
                    self.alive = false;
                    return Ok(());
                }    
            }
            self.last_update = time::Instant::now();
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.1, 0.0, 0.5].into());
        let background = graphics::Image::new(ctx, "/grass400x440.jpg").unwrap();   // zła praktyka
        graphics::draw(ctx, &background, graphics::DrawParam::default())?;
        self.draw_grid(ctx)?;
        self.snake.draw(ctx, self.last_meal)?;
        let _ = match &self.food_cell{
            Some(f) => f.draw(ctx),
            None => Ok(()),
        };
        let down_bar = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, MAP_SIZE_Y, MAP_SIZE_Y, CELL_SIZE as f32),
            [0.0, 0.0, 0.0, 0.7].into())?;
        graphics::draw(ctx, &down_bar, (na::Point2::new(0.0, 0.0), ))?;

        let text_to_display: String;
        if self.menu{
            text_to_display = "PRESS ENTER TO START".to_string();
            self.text_location = CELL_SIZE as f32;
        }
        else if !self.alive{
            text_to_display = "PRESS SPACE TO RESTART".to_string();
            let points_text = graphics::Text::new(graphics::TextFragment{
                text: "SCORED POINTS: ".to_string() + &self.points.to_string(),
                color: Some(graphics::WHITE),
                font: Some(graphics::Font::default()),
                scale: Some(graphics::Scale::uniform(38.0)),
            });
            graphics::draw(ctx, &points_text, (na::Point2::new(self.text_location - 4.0, MAP_SIZE_Y / 2.5),))?;
            self.text_location = CELL_SIZE as f32 / 2.0;
        }
        else{
            text_to_display = "POINTS: ".to_string() + &self.points.to_string();
            self.text_location = (3 * CELL_SIZE) as f32;
        }
        let text = graphics::Text::new(graphics::TextFragment{
            text: text_to_display,
            color: Some(TEXT_COLOR.into()),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::Scale::uniform(29.0)),
        });
        graphics::draw(ctx, &text, (na::Point2::new(self.text_location, MAP_SIZE_Y + 2.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self, 
        _ctx: &mut ggez::Context, 
        keycode: event::KeyCode, 
        _keymod: event::KeyMods,
        _repeat: bool){
            let new_direction: String = match keycode{
                event::KeyCode::W => "UP".to_string(),
                event::KeyCode::S => "DOWN".to_string(),
                event::KeyCode::A => "LEFT".to_string(),
                event::KeyCode::D => "RIGHT".to_string(),
                _ => self.snake.direction.clone(),
            };
            if !self.snake.opposite_direction_to(&new_direction){
                self.snake.new_direction = new_direction;
            }   
        }
}

fn main() -> ggez::GameResult{
    let cb = ggez::ContextBuilder::new("draw a line", "ggez").add_resource_path(path::PathBuf::from("./resources"));
    let (ctx, event_loop) = &mut cb
        .window_mode(ggez::conf::WindowMode::default().dimensions(MAP_SIZE_X, MAP_SIZE_Y + CELL_SIZE as f32))
        .build()?;
    let state = &mut MainState::new(START_POS_X, START_POS_Y)?;
    event::run(ctx, event_loop, state)
}
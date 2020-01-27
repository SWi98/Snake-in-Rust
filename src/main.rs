mod snake;
mod position_on_map;

use snake::Snake;
use position_on_map::PositionOnMap;

use ggez;
use ggez::{event, graphics, nalgebra as na};
use std::time;
use std::collections::LinkedList;

const MAP_SIZE_X: f32 = 400.0;
const MAP_SIZE_Y: f32 = 400.0;
const CELL_SIZE: i32 = 20;
const START_POS_X: i32 = 0;
const START_POS_Y: i32 = 0;
const ROUND_TIME: u64 = 500;      //ms


pub struct MainState{
    last_update: time::Instant,
    snake: Snake,
    alive: bool,
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
        let s = MainState{
            last_update: time::Instant::now(),
            snake: sn,
            alive: true,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState{
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult{

        // If snake has collided with something, we change its color don't update its position
        if !self.alive{
            self.snake.col = [1.0, 0.0, 0.0, 1.0].into();
        }

        // Checking if enough time has passed since the last update
        else if time::Instant::now() - self.last_update >= time::Duration::from_millis(ROUND_TIME){
            // Checking if snake's new direction differs from its current direction
            /*if self.snake.new_direction != self.snake.direction && !self.snake.opposite_direction(){
                self.snake.direction = self.snake.new_direction.clone();
            }*/
            self.snake.direction = self.snake.new_direction.clone();
            let head = self.snake.segments.front().unwrap();

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

            print!("Head: {}, {}; time elapsed: {}; Segmenty:", head.pos_x, head.pos_y, self.last_update.elapsed().as_millis());
            for segment in self.snake.segments.iter(){
                 print!("{}, {}; ", segment.pos_x, segment.pos_y);
            }
            println!("");

            if new_x != head.pos_x || new_y != head.pos_y{              // Checking if the snake has moved
                let new_pos = PositionOnMap{pos_x: new_x, pos_y: new_y}; 
                self.snake.segments.pop_back();             // Moving the snake  
                self.snake.segments.push_front(new_pos); 
                println!("{}", self.snake.collide());
                if self.snake.collide(){    // Checking if the snake collided
                    self.alive = false;
                    return Ok(());
                }    
            }
            
            self.last_update = time::Instant::now();
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.snake.draw(ctx, self.snake.col)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self, 
        ctx: &mut ggez::Context, 
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
    let cb = ggez::ContextBuilder::new("draw a line", "ggez");
    let (ctx, event_loop) = &mut cb
        .window_mode(ggez::conf::WindowMode::default().dimensions(MAP_SIZE_X, MAP_SIZE_Y))
        .build()?;
    let state = &mut MainState::new(START_POS_X, START_POS_Y)?;
    event::run(ctx, event_loop, state)
}
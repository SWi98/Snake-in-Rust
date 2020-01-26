use ggez;
use ggez::{event, graphics, input, nalgebra as na};
use std::time;
use std::collections::LinkedList;

const MAP_SIZE_X: f32 = 400.0;
const MAP_SIZE_Y: f32 = 400.0;
const CELL_SIZE: i32 = 20;
const START_POS_X: i32 = 0;
const START_POS_Y: i32 = 0;
const ROUND_TIME: u64 = 400;      //ms

pub struct PositionOnMap{
    pos_x: i32,
    pos_y: i32,
}

pub struct Snake{
    segments: LinkedList<PositionOnMap>,
    direction: String,
}

impl Snake{
    fn new_poss(pos_x: i32, pos_y: i32) -> ggez::GameResult<Snake>{
        let mut snake = LinkedList::new();
        snake.push_back(PositionOnMap{pos_x: pos_x, pos_y: pos_y});
        let s = Snake{segments: snake, direction: "RIGHT".to_string()};
        Ok(s)
    }

    fn new_snake(snake: LinkedList<PositionOnMap>) -> ggez::GameResult<Snake>{
        let s = Snake{segments: snake, direction: "RIGHT".to_string()};
        Ok(s)
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult{
        for segment in self.segments.iter(){
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    segment.pos_x as f32, 
                    segment.pos_y as f32, 
                    CELL_SIZE as f32, 
                    CELL_SIZE as f32),
                graphics::WHITE)?;
            graphics::draw(ctx, &rect, (na::Point2::new(0.0, 0.0), ))?;
        }
        Ok(())
    }

    fn opposite_direction(&mut self, new_direction: &String) -> bool{
        (self.direction == "UP" && new_direction == "DOWN") || 
        (self.direction == "LEFT" && new_direction == "RIGHT") || 
        (self.direction == "DOWN" && new_direction == "UP") ||
        (self.direction == "RIGHT" && new_direction == "LEFT")
    }
}

pub struct MainState{
    last_update: time::Instant,
    snake: Snake,
}

impl MainState{
    fn new(pos_x: i32, pos_y: i32) -> ggez::GameResult<MainState>{
        let mut segm = LinkedList::new();
        segm.push_front(PositionOnMap{pos_x: pos_x, pos_y: pos_y});
        segm.push_front(PositionOnMap{pos_x: pos_x + CELL_SIZE, pos_y: pos_y});
        segm.push_front(PositionOnMap{pos_x: pos_x + CELL_SIZE * 2, pos_y: pos_y});
        segm.push_front(PositionOnMap{pos_x: pos_x + CELL_SIZE * 3, pos_y: pos_y});
        let sn = Snake::new_snake(segm)?;
        let s = MainState{
            last_update: time::Instant::now(),
            snake: sn,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState{
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult{
        if time::Instant::now() - self.last_update >= time::Duration::from_millis(ROUND_TIME){
            let head = self.snake.segments.front().unwrap();
            let mut new_x: i32 = head.pos_x;
            let mut new_y: i32 = head.pos_y;

            if self.snake.direction == "UP" && head.pos_y != 0{
                new_y = head.pos_y - CELL_SIZE;
            }
            else if self.snake.direction == "DOWN" && head.pos_y != 400 - CELL_SIZE{
                new_y = head.pos_y + CELL_SIZE;
            }
            else if self.snake.direction == "LEFT" && head.pos_x != 0{
                new_x = head.pos_x - CELL_SIZE;
            }
            else if self.snake.direction == "RIGHT" && head.pos_x != 400 - CELL_SIZE{
                new_x = head.pos_x + CELL_SIZE;
            }

            if new_x != head.pos_x || new_y != head.pos_y{
               // println!("{}, {}", new_x, new_y);
                self.snake.segments.push_front(PositionOnMap{pos_x: new_x, pos_y: new_y});
                self.snake.segments.pop_back();
               // println!("CHANGED");
            }
            
            let head = self.snake.segments.front().unwrap();
            self.last_update = time::Instant::now();
            print!("Head: {}, {}; time elapsed: {}; Segmenty:", head.pos_x, head.pos_y, self.last_update.elapsed().as_millis());
            for segment in self.snake.segments.iter(){
                print!("{}, {}; ", segment.pos_x, segment.pos_y);
            }
            println!("");
        }
        Ok(())
        
    }
    
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.snake.draw(ctx)?;
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
            if new_direction != self.snake.direction && !self.snake.opposite_direction(&new_direction){
                self.snake.direction = new_direction;
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
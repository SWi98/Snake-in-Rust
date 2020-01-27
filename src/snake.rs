use ggez;
use ggez::{event, graphics, nalgebra as na};
use std::time;
use std::collections::LinkedList;
use crate::position_on_map::PositionOnMap;

pub struct Snake{
    pub segments: LinkedList<PositionOnMap>,
    pub direction: String,
    pub new_direction: String,
    pub col: graphics::Color,
}

impl Snake{
    pub fn new_poss(pos_x: i32, pos_y: i32) -> ggez::GameResult<Snake>{
        let mut snake = LinkedList::new();
        snake.push_back(PositionOnMap{pos_x: pos_x, pos_y: pos_y});
        let s = Snake{
            segments: snake, 
            direction: "RIGHT".to_string(), 
            new_direction: "RIGHT".to_string(),
            col: graphics::WHITE,
        };
        Ok(s)
    }

    pub fn new_snake(snake: LinkedList<PositionOnMap>) -> ggez::GameResult<Snake>{
        let s = Snake{segments: snake, 
            direction: "RIGHT".to_string(), 
            new_direction: "RIGHT".to_string(),
            col: graphics::WHITE,
        };
        Ok(s)
    }

    pub fn collide(&self) -> bool{
        let head: &PositionOnMap = self.segments.front().unwrap();
        if head.out_of_map(){
            return true;
        }
        let mut collided_segments: i16 = 0;
        for segment in self.segments.iter(){
            if segment.pos_x == head.pos_x && segment.pos_y == head.pos_y{
                collided_segments += 1;
            }
            if collided_segments > 1{
                return true;
            }
        }
        return false;
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context, col: graphics::Color) -> ggez::GameResult{
        for segment in self.segments.iter(){
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    segment.pos_x as f32, 
                    segment.pos_y as f32, 
                    super::CELL_SIZE as f32, 
                    super::CELL_SIZE as f32),
                col)?;
            graphics::draw(ctx, &rect, (na::Point2::new(0.0, 0.0), ))?;
        }
        Ok(())
    }

    pub fn opposite_direction(&self) -> bool{
        (self.direction == "UP" && self.new_direction == "DOWN") || 
        (self.direction == "LEFT" && self.new_direction == "RIGHT") || 
        (self.direction == "DOWN" && self.new_direction == "UP") ||
        (self.direction == "RIGHT" && self.new_direction == "LEFT")
    }
    
    pub fn opposite_direction_to(&self, other: &String) -> bool{
        (self.direction == "UP" && other == "DOWN") || 
        (self.direction == "LEFT" && other == "RIGHT") || 
        (self.direction == "DOWN" && other == "UP") ||
        (self.direction == "RIGHT" && other == "LEFT")
    }
}

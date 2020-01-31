use ggez;
use ggez::{graphics, nalgebra as na};
use std::collections::LinkedList;
use crate::position_on_map::PositionOnMap;
use super::*;

pub struct Snake{
    segments: LinkedList<PositionOnMap>,
    pub direction: String,
    pub new_direction: String,
    pub col: graphics::Color,
}

impl Snake{
    pub fn new_snake(snake: LinkedList<PositionOnMap>) -> ggez::GameResult<Snake>{
        let s = Snake{segments: snake, 
            direction: "RIGHT".to_string(), 
            new_direction: "RIGHT".to_string(),
            col: [0.2, 0.1, 0.3, 1.0].into(),//graphics::WHITE,
        };
        Ok(s)
    }

    pub fn move_head(&mut self, new_pos: PositionOnMap){
        self.segments.push_front(new_pos);
    }

    pub fn move_tail(&mut self){
        self.segments.pop_back();
    }

    pub fn get_head(&self) -> Option<&PositionOnMap>{
        self.segments.front()
    }

    pub fn collide_with_food(&self, new_food: &Food) -> bool{
        self.segments.contains(&new_food.get_pos())
    }

    pub fn collide_with_pickup(&self, new_pickup: &PickUp) -> bool{
        self.segments.contains(&new_pickup.get_pos())
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
                    segment.pos_x as f32 + 2.0, 
                    segment.pos_y as f32 + 2.0, 
                    CELL_SIZE as f32 - 4.0, 
                    CELL_SIZE as f32 - 4.0),
                col)?;
            graphics::draw(ctx, &rect, (na::Point2::new(0.0, 0.0), ))?;
        }
        Ok(())
    }
    
    pub fn opposite_direction_to(&self, other: &String) -> bool{
        (self.direction == "UP" && other == "DOWN") || 
        (self.direction == "LEFT" && other == "RIGHT") || 
        (self.direction == "DOWN" && other == "UP") ||
        (self.direction == "RIGHT" && other == "LEFT")
    }
}

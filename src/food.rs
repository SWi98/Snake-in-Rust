extern crate rand;

use ggez;
use ggez::{graphics, nalgebra as na};
use crate::position_on_map::PositionOnMap;
use super::*;

pub struct Food{
    position: PositionOnMap,
}

impl Food{
    pub fn new_random() -> Food{
        let pos = PositionOnMap::new_random();
        return Food{position: PositionOnMap{pos_x: pos.pos_x, pos_y: pos.pos_y}};
    }

    pub fn get_pos(&self) -> &PositionOnMap{
        &self.position
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult{
        /*let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.position.pos_x as f32,
                self.position.pos_y as f32,
                CELL_SIZE as f32, 
                CELL_SIZE as f32),
            [1.0, 1.0, 0.0, 1.0].into())?;*/
            
        let coin = graphics::Image::new(ctx, "/coin40x40.png").unwrap();
        graphics::draw(ctx, &coin, (na::Point2::new(self.position.pos_x as f32, self.position.pos_y as f32),))?;
        Ok(())
    }
}

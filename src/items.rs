use crate::position_on_map::PositionOnMap;
use crate::ggez;
use super::*;

const EFFECT_TYPES: [&str; 1] = ["SPEED"];

pub struct PickUp{
    position: PositionOnMap,
    effect: usize,
    duration: u32,
}

impl PickUp{
    pub fn new_random() -> PickUp{
        let pos = PositionOnMap::new_random();
        PickUp{position: pos, effect: 0, duration: 500}
    }

    pub fn get_pos(&self) -> &PositionOnMap{
        &self.position
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult{
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.position.pos_x as f32 + 8.0,
                self.position.pos_y as f32 + 8.0,
                CELL_SIZE as f32 - 16.0, 
                CELL_SIZE as f32 - 16.0),
            [0.0, 0.0, 1.0, 1.0].into())?;
        graphics::draw(ctx, &rect, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }

    pub fn get_type(&self) -> &str{
        EFFECT_TYPES[self.effect]
    }
}

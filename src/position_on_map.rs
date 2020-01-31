use super::*;
use rand::distributions::{Uniform, Distribution};

pub struct PositionOnMap{
    pub pos_x: i32,
    pub pos_y: i32,
}

impl PositionOnMap{
    pub fn new_random() -> PositionOnMap{
        let stepx = Uniform::new(0, MAP_SIZE_X as i32);
        let stepy = Uniform::new(0, MAP_SIZE_Y as i32);
        let mut rng = rand::thread_rng();
        let mut x: i32 = stepx.sample(&mut rng);
        let mut y: i32 = stepy.sample(&mut rng);
        x = x / super::CELL_SIZE * super::CELL_SIZE;
        y = y / super::CELL_SIZE * super::CELL_SIZE;
        PositionOnMap{pos_x: x, pos_y: y}
    }

    pub fn out_of_map(&self) -> bool{
        return self.pos_x < 0 || self.pos_x >= MAP_SIZE_X as i32 ||
             self.pos_y < 0 || self.pos_y >= MAP_SIZE_Y as i32;
    }
}

impl PartialEq for PositionOnMap{

    fn eq(&self, other: &PositionOnMap) -> bool{
        self.pos_x == other.pos_x && self.pos_y == other.pos_y
    }
}
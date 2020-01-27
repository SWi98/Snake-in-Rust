

pub struct PositionOnMap{
    pub pos_x: i32,
    pub pos_y: i32,
}

impl PositionOnMap{
    pub fn out_of_map(&self) -> bool{
        return self.pos_x < 0 || self.pos_x > super::MAP_SIZE_X as i32 ||
             self.pos_y < 0 || self.pos_y > super::MAP_SIZE_Y as i32;
    }
}

impl PartialEq for PositionOnMap{
    fn eq(&self, other: &PositionOnMap) -> bool{
        self.pos_x == other.pos_x && self.pos_y == other.pos_y
    }
}
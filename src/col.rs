use nogine::color::Color4;

use crate::{aabb::AABB, slope::Slope};

#[derive(Clone, Copy)]
pub enum Col {
    AABB(AABB),
    Slope(Slope),
}

impl Col {
    pub fn bounding_box(&self) -> AABB {
        match self {
            Col::AABB(x) => *x,
            Col::Slope(x) => x.bounding_box(),
        }
    }

    pub fn draw(&self, color: Color4) {
        match self {
            Col::AABB(x) => x.draw(color),
            Col::Slope(x) => x.draw(color),
        }
    }
}
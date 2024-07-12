use std::ops::{Add, BitOr};

use nogine::{color::Color4, graphics::Graphics, math::Vector2};

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vector2,
    pub max: Vector2,
}

impl AABB {
    pub fn draw(&self, color: Color4) {
        Graphics::draw_line(self.min, self.min.xvec() + self.max.yvec(), color);
        Graphics::draw_line(self.min.xvec() + self.max.yvec(), self.max, color);
        Graphics::draw_line(self.max.xvec() + self.min.yvec(), self.max, color);
        Graphics::draw_line(self.min, self.max.xvec() + self.min.yvec(), color);
    }

    pub fn draw_connect(&self, end: Self, color: Color4) {
        Graphics::draw_line(self.ld(), end.ld(), color);
        Graphics::draw_line(self.rd(), end.rd(), color);
        Graphics::draw_line(self.lu(), end.lu(), color);
        Graphics::draw_line(self.ru(), end.ru(), color);
    }

    pub fn lu(&self) -> Vector2 {
        Vector2(self.min.0, self.max.1)
    }

    pub fn ru(&self) -> Vector2 {
        Vector2(self.max.0, self.max.1)
    }

    pub fn rd(&self) -> Vector2 {
        Vector2(self.max.0, self.min.1)
    }

    pub fn ld(&self) -> Vector2 {
        Vector2(self.min.0, self.min.1)
    }

    pub fn intersects(&self, other: Self) -> bool {
        return
            self.min.0 < other.max.0 &&
            self.max.0 > other.min.0 &&
            self.min.1 < other.max.1 &&
            self.max.1 > other.min.1;
    }
    
    pub fn center(&self) -> Vector2 {
        (self.min + self.max) * 0.5
    }
}

impl BitOr for AABB {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let min = self.min.min(rhs.min);
        let max = self.max.max(rhs.max);
        return Self { min, max };
    }
}

impl Add<Vector2> for AABB {
    type Output = Self;

    fn add(self, rhs: Vector2) -> Self::Output {
        return Self { min: self.min + rhs, max: self.max + rhs };
    }
}
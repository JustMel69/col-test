use nogine::{color::Color4, graphics::Graphics, math::Vector2};

use crate::{aabb::AABB, shapecast::{HitType, Segment}};

#[derive(Clone, Copy)]
pub enum SlopeNormal {
    LU, RU, RD, LD
}

impl SlopeNormal {
    pub fn next(&mut self) {
        match self {
            SlopeNormal::LU => *self = SlopeNormal::RU,
            SlopeNormal::RU => *self = SlopeNormal::RD,
            SlopeNormal::RD => *self = SlopeNormal::LD,
            SlopeNormal::LD => *self = SlopeNormal::LU,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Slope {
    pub aabb: AABB,
    pub normal: SlopeNormal,
}

impl Slope {
    pub fn bounding_box(&self) -> AABB {
        self.aabb
    }

    pub fn draw(&self, color: Color4) {
        match self.normal {
            SlopeNormal::LU => {
                Graphics::draw_line(self.aabb.ld(), self.aabb.rd(), color);
                Graphics::draw_line(self.aabb.rd(), self.aabb.ru(), color);
                Graphics::draw_line(self.aabb.ld(), self.aabb.ru(), color);
            },
            SlopeNormal::RU => {
                Graphics::draw_line(self.aabb.ld(), self.aabb.rd(), color);
                Graphics::draw_line(self.aabb.ld(), self.aabb.lu(), color);
                Graphics::draw_line(self.aabb.rd(), self.aabb.lu(), color);
            },
            SlopeNormal::RD => {
                Graphics::draw_line(self.aabb.lu(), self.aabb.ru(), color);
                Graphics::draw_line(self.aabb.ld(), self.aabb.lu(), color);
                Graphics::draw_line(self.aabb.ld(), self.aabb.ru(), color);
            },
            SlopeNormal::LD => {
                Graphics::draw_line(self.aabb.lu(), self.aabb.ru(), color);
                Graphics::draw_line(self.aabb.rd(), self.aabb.ru(), color);
                Graphics::draw_line(self.aabb.lu(), self.aabb.rd(), color);
            },
        }
    }

    pub fn straight_hits(&self) -> (HitType, HitType) {
        match self.normal {
            SlopeNormal::LU => (HitType::UtD, HitType::LtR),
            SlopeNormal::RU => (HitType::UtD, HitType::RtL),
            SlopeNormal::RD => (HitType::DtU, HitType::RtL),
            SlopeNormal::LD => (HitType::DtU, HitType::LtR),
        }
    }

    pub fn slope_vec_normal(&self) -> Vector2 {
        let raw_normal = match self.normal {
            SlopeNormal::LU => Vector2(-1.0,  1.0).normalized(),
            SlopeNormal::RU => Vector2( 1.0,  1.0).normalized(),
            SlopeNormal::RD => Vector2( 1.0, -1.0).normalized(),
            SlopeNormal::LD => Vector2(-1.0, -1.0).normalized(),
        };

        let precision_scaling = (self.aabb.max - self.aabb.min).max_axis().max(1.0); // Multiply by this number so there are less precision errors when shrinking the vector

        return (raw_normal * precision_scaling).inv_scale(self.aabb.max - self.aabb.min).normalized();
    }
    
    pub fn slope_segment(&self) -> Segment {
        match self.normal {
            SlopeNormal::LU | SlopeNormal::RD => Segment { start: self.aabb.ld(), end: self.aabb.ru() },
            SlopeNormal::RU | SlopeNormal::LD => Segment { start: self.aabb.rd(), end: self.aabb.lu() },
        }
    }

    
}
use nogine::math::Vector2;

use crate::{aabb::AABB, col::Col, slope::{Slope, SlopeNormal}};


#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HitType {
    LtR, RtL, UtD, DtU, Slope, Inside,
}

const NORMALS: [Vector2; 6] = [ Vector2::RIGHT, Vector2::LEFT, Vector2::DOWN, Vector2::UP, Vector2::ZERO, Vector2::ZERO ];

pub struct ShapecastResult {
    /// Hit type.
    pub hit_type: HitType,
    
    /// Hit normal.
    pub normal: Vector2,
    
    // Hit distance.
    pub distance: f32,
}

pub fn subshapecast(dy: AABB, st: Col, delta: Vector2) -> Option<ShapecastResult> {
    let broad = dy | (dy + delta);

    // Broad Phasing
    if broad.intersects(st.bounding_box()) {
        return match st {
            Col::AABB(st) => aabb_x_aabb(dy, st, delta),
            Col::Slope(st) => aabb_x_tri(dy, st, delta),
        };
    }

    return None;
}

fn aabb_x_aabb(dy: AABB, st: AABB, delta: Vector2) -> Option<ShapecastResult> {
    // Inside collider test
    if dy.intersects(st) {
        return Some(ShapecastResult { hit_type: HitType::Inside, normal: Vector2::ZERO, distance: 0.0 });
    }
    
    // Distance in each axis needed to begin contact
    let dist_entry = Vector2(
        if delta.0 > 0.0 { st.min.0 - dy.max.0 } else { st.max.0 - dy.min.0 },
        if delta.1 > 0.0 { st.min.1 - dy.max.1 } else { st.max.1 - dy.min.1 },
    );

    // Distance in each axis needed to exit contaxt
    let dist_exit = Vector2(
        if delta.0 <= 0.0 { st.min.0 - dy.max.0 } else { st.max.0 - dy.min.0 },
        if delta.1 <= 0.0 { st.min.1 - dy.max.1 } else { st.max.1 - dy.min.1 },
    );

    // Progress in delta at entry time for each axis
    let entry_fact = Vector2(
        if delta.0 == 0.0 { f32::NEG_INFINITY } else { dist_entry.0 / delta.0 },
        if delta.1 == 0.0 { f32::NEG_INFINITY } else { dist_entry.1 / delta.1 },
    );

    // Progress in delta at exit time for each axis
    let exit_fact = Vector2(
        if delta.0 == 0.0 { f32::INFINITY } else { dist_exit.0 / delta.0 },
        if delta.1 == 0.0 { f32::INFINITY } else { dist_exit.1 / delta.1 },
    );

    let entry = entry_fact.max_axis(); // Max because both axis need to have collided
    let exit = exit_fact.min_axis(); // Min because only one axis needs to have left collision

    if entry < 0.0 || entry > 1.0 || exit <= entry {
        return None; // No collision
    }


    let hit_type = if entry_fact.0 > entry_fact.1 {
        if delta.0 > 0.0 { HitType::RtL } else { HitType::LtR } // Horizontal hit
    } else {
        if delta.1 > 0.0 { HitType::UtD } else { HitType::DtU } // Vertical hit
    };

    return Some(ShapecastResult {
        hit_type,
        normal: NORMALS[hit_type as usize],
        distance: delta.magnitude() * entry,
    });
}

fn aabb_x_tri(dy: AABB, st: Slope, delta: Vector2) -> Option<ShapecastResult> {
    let subres = aabb_x_aabb(dy, st.aabb, delta)?;

    // Propagate if not gonna touch the slope
    let straights = st.straight_hits();
    if subres.hit_type == straights.0 || subres.hit_type == straights.1 {
        return Some(subres);
    }

    // Propagate if colliding with the tips
    if tipcheck(dy + delta.normalized() * subres.distance, st, subres.hit_type) {
        return Some(subres);
    }

    // Check with slope
    let start_vert = match st.normal { // An AABB will only touch a slope with one (1) vert.
        SlopeNormal::LU => dy.rd(),
        SlopeNormal::RU => dy.ld(),
        SlopeNormal::RD => dy.lu(),
        SlopeNormal::LD => dy.ru(),
    };

    let end_vert = start_vert + delta;

    let dyn_seg = Segment { start: start_vert, end: end_vert };
    let st_seg = st.slope_segment();

    // Checks for intersections
    let hit = dyn_seg.intersection(&st_seg)?;
    let distance = (hit - start_vert).magnitude();

    return Some(ShapecastResult {
        hit_type: HitType::Slope,
        normal: st.slope_vec_normal(),
        distance,
    });
}

fn tipcheck(hit_dy: AABB, st: Slope, hit_type: HitType) -> bool {
    match st.normal {
        SlopeNormal::LU => {
            match hit_type {
                HitType::RtL => hit_dy.min.1 < st.aabb.min.1,
                HitType::DtU => hit_dy.max.0 > st.aabb.max.0,
                _ => false
            }
        },
        SlopeNormal::RU => {
            match hit_type {
                HitType::LtR => hit_dy.min.1 < st.aabb.min.1,
                HitType::DtU => hit_dy.min.0 < st.aabb.min.0,
                _ => false
            }
        },
        SlopeNormal::RD => {
            match hit_type {
                HitType::LtR => hit_dy.max.1 > st.aabb.max.1,
                HitType::UtD => hit_dy.min.0 < st.aabb.min.0,
                _ => false
            }
        },
        SlopeNormal::LD => {
            match hit_type {
                HitType::RtL => hit_dy.max.1 > st.aabb.max.1,
                HitType::UtD => hit_dy.max.0 > st.aabb.max.0,
                _ => false
            }
        },
    }
}


pub struct Segment {
    pub start: Vector2, pub end: Vector2
}

impl Segment {
    pub fn intersection(&self, other: &Self) -> Option<Vector2> {
        let s_delta = self.end - self.start;
        let o_delta = other.end - other.start;
        
        let b = o_delta.0 * s_delta.1 - o_delta.1 * s_delta.0;

        if b == 0.0 {
            return None; // Parallel lines
        }

        let a = o_delta.0 * (other.start.1 - self.start.1) - o_delta.1 * (other.start.0 - self.start.0);
        let c = s_delta.0 * (other.start.1 - self.start.1) - s_delta.1 * (other.start.0 - self.start.0);
        
        let alpha = a / b;
        let beta = c / b;

        if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 {
            return None;
        }

        return Some(self.start + s_delta * alpha);
    }
}
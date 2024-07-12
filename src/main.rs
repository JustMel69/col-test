use aabb::AABB;
use col::Col;
use nogine::{color::{Color, Color4}, graphics::Graphics, input::{Input, MouseInput}, math::Vector2, window::WindowCfg};
use shapecast::subshapecast;
use slope::Slope;

mod aabb;
mod col;
mod slope;
mod shapecast;

enum State {
    Standby,
    Drag,
    Move,
}

fn main() {
    let mut window = WindowCfg::default().title("Collision Test").res((1280, 720)).init().unwrap();
    window.set_resizable(false);
    window.set_vsync(true);

    let mut st_cols = [
        Col::AABB(AABB { min: Vector2(-4.0, 3.0), max: Vector2(4.0, 6.0) }),
        Col::Slope(Slope { aabb: AABB { min: Vector2(-3.0, -6.0), max: Vector2(3.0, -3.0) }, normal: slope::SlopeNormal::LD }),
    ];

    let mut state = State::Standby;
    let mut aabb: Option<(AABB, Vector2)> = None;

    let mut t = 0.0;

    while window.is_running() {
        window.pre_tick(None);

        t += window.ts();
        if t > 4.0 {
            if let Col::Slope(s) = &mut st_cols[1] {
                s.normal.next();
            }
            t = 0.0;
        }

        Graphics::set_cam(Vector2::ZERO, Vector2(10.0 * window.aspect_ratio(), 10.0));

        for c in st_cols {
            c.bounding_box().draw(Color4::DARK_GRAY);
            c.draw(Color4::WHITE);
        }

        draw_cross(Vector2::ZERO, Color4::DARK_GRAY);

        match state {
            State::Standby => {
                if Input::mouse_pressed(MouseInput::Left) {
                    state = State::Drag;
                    let mouse = mouse_pos();
                    aabb = Some((AABB { min: mouse, max: mouse }, Vector2::ZERO));
                }
            },
            State::Drag => {
                aabb.as_mut().unwrap().0.max = mouse_pos();

                if Input::mouse_released(MouseInput::Left) {
                    state = State::Move;
                    
                    let col = aabb.unwrap().0;
                    let min = col.min.min(col.max);
                    let max = col.min.max(col.max);
                    
                    aabb = Some((AABB { min, max }, Vector2::ZERO));
                }
            },
            State::Move => {
                let col = aabb.unwrap().0;
                aabb = Some((col, mouse_pos() - col.max));

                if Input::mouse_pressed(MouseInput::Left) {
                    state = State::Standby;
                }
            },
        }

        if let Some((x, d)) = aabb {
            let end = x + d;

            x.draw(Color4::LIME);

            let mut hit_res = None;
            let mut delta = d;
            for c in st_cols {
                if let Some(sub_hit_res) = subshapecast(x, c, delta) {
                    delta = d.normalized() * sub_hit_res.distance;
                    hit_res = Some(sub_hit_res);
                }
            }

            if hit_res.is_some() {
                let mid = x + delta;
                x.draw_connect(mid, Color4::YELLOW);
                mid.draw(Color4::PINK);
                mid.draw_connect(end, Color4::DARK_GRAY);
                end.draw(Color4::GRAY);

                draw_arrow(mid.center(), hit_res.unwrap().normal, Color4::PINK);
            } else {
                x.draw_connect(end, Color4::YELLOW);
                end.draw(Color4::PINK);
            }
        }

        draw_cross(mouse_pos(), Color4::CYAN);

        window.post_tick();
    }
}

fn draw_cross(point: Vector2, color: Color4) {
    Graphics::draw_line(point + Vector2(-0.25, 0.25), point + Vector2(0.25, -0.25), color);
    Graphics::draw_line(point + Vector2::one(-0.25), point + Vector2::one(0.25), color);
}

fn mouse_pos() -> Vector2 {
    let ss = Input::mouse_pos();
    let ws = ss.inv_scale(Vector2(1280.0, 720.0)).scale(Vector2(16.0 / 9.0 * 20.0, 20.0)) - Vector2(10.0 * 16.0 / 9.0, 10.0);
    return Vector2(ws.0, -ws.1);
}

fn draw_arrow(origin: Vector2, dir: Vector2, color: Color4) {
    let x = dir.cross();
    let y = dir;

    Graphics::draw_line(origin, origin + y, color);
    Graphics::draw_line(origin + y, origin + y * 0.75 - x * 0.25, color);
    Graphics::draw_line(origin + y, origin + y * 0.75 + x * 0.25, color);
}
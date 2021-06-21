use graphics::math::Matrix2d;
use graphics::{DrawState, Graphics, Line};

pub struct Ray {
    x: f64,
    y: f64,
    dir: (f64, f64),
    line: graphics::Line,
}

impl Ray {
    pub fn new(x: f64, y: f64) -> Ray {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        Ray {
            x,
            y,
            dir: (1.0, 0.0),
            line: graphics::Line::new(WHITE, 1.0),
        }
    }

    pub fn draw<G: Graphics>(&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G) {
        Line::draw(
            &self.line,
            [
                self.x,
                self.y,
                self.dir.0 * 10.0 + self.x,
                self.dir.1 * 10.0 + self.y,
            ],
            draw_state,
            transform,
            g,
        );
    }
}

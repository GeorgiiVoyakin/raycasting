use graphics::math::Matrix2d;
use graphics::{DrawState, Graphics, Line};

use crate::boundary::Boundary;
use crate::point::Point;

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

    pub fn cast(&self, boundary: Boundary) -> Option<Point> {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = self.x + self.dir.0 * 10.0;
        let y2 = self.y + self.dir.1 * 10.0;

        let x3 = boundary.coords()[0];
        let y3 = boundary.coords()[1];
        let x4 = boundary.coords()[2];
        let y4 = boundary.coords()[3];

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denominator == 0.0 {
            return None;
        }
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
        let u = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denominator;

        if t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0 {
            let point_x = x1 + t * (x2 - x1);
            let point_y = y1 + t * (y2 - y1);
            Some(Point::new(point_x, point_y, 10.0))
        } else {
            None
        }
    }
}

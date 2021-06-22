use graphics::math::Matrix2d;
use graphics::{DrawState, Ellipse, Graphics};

pub struct Point {
    _x: f64,
    _y: f64,
    _radius: f64,
    ellipse: graphics::Ellipse,
    rect: [f64; 4],
}

impl Point {
    pub fn new(x: f64, y: f64, radius: f64) -> Point {
        let color = [0.7, 0.7, 0.7, 1.0];

        Point {
            _x: x,
            _y: y,
            _radius: radius,
            ellipse: graphics::Ellipse::new(color),
            rect: [x - radius, y - radius, 2.0 * radius, 2.0 * radius],
        }
    }

    pub fn draw<G: Graphics>(&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G) {
        Ellipse::draw(
            &self.ellipse, 
            self.rect, 
            draw_state, 
            transform, 
            g
        );
    }
}

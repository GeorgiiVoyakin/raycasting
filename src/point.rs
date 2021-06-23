use graphics::math::Matrix2d;
use graphics::{DrawState, Ellipse, Graphics};

#[derive(Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    _radius: f64,
    _ellipse: graphics::Ellipse,
    _rect: [f64; 4],
}

impl Point {
    pub fn new(x: f64, y: f64, radius: f64) -> Point {
        let color = [0.7, 0.7, 0.7, 1.0];

        Point {
            x,
            y,
            _radius: radius,
            _ellipse: graphics::Ellipse::new(color),
            _rect: [x - radius, y - radius, 2.0 * radius, 2.0 * radius],
        }
    }

    pub fn _draw<G: Graphics>(&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G) {
        Ellipse::draw(
            &self._ellipse, 
            self._rect, 
            draw_state, 
            transform, 
            g
        );
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

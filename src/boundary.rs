use graphics::{Graphics, DrawState, Line};
use graphics::math::Matrix2d;

pub struct Boundary {
    line_coords: [f64; 4],
    line: graphics::Line,
}

impl Boundary {
    pub fn new(line_coords: [f64; 4]) -> Boundary {
        const BOUNDARY_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

        Boundary {
            line_coords,
            line: graphics::Line::new(BOUNDARY_COLOR, 1.0),
        }
    }

    pub fn draw<G: Graphics>(&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G) {
        Line::draw(&self.line,
                   self.line_coords,
                   draw_state,
                   transform,
                   g);
    }

    pub fn coords(&self) -> [f64; 4] { 
        self.line_coords 
    }
}
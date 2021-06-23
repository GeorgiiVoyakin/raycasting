extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{MouseCursorEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use crate::boundary::Boundary;
use crate::point::Point;
use crate::ray::Ray;

mod boundary;
mod point;
mod ray;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    mouse_pos: [f64; 2],
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        let boundary: Boundary = Boundary::new([500.0, 20.0, 500.0, 400.0]);
        let boundary2: Boundary = Boundary::new([0.0, 300.0, 800.0, 300.0]);
        let mut boundaries: Vec<Boundary> = Vec::new();
        let mut rays: Vec<Ray> = Vec::new();

        for i in 0..360 {
            rays.push(Ray::new(
                self.mouse_pos[0],
                self.mouse_pos[1],
                (
                    (i as f64 * (std::f64::consts::PI / 180 as f64)).cos(),
                    (i as f64 * (std::f64::consts::PI / 180 as f64)).sin(),
                ),
            ));
        }

        boundaries.push(boundary);
        boundaries.push(boundary2);

        self.gl.draw(args.viewport(), |c, gl| {
            for b in boundaries.iter() {
                b.draw(&c.draw_state, c.transform, gl);
            }

            for r in rays.iter() {
                r.draw(&c.draw_state, c.transform, gl);

                for b in boundaries.iter() {
                    let point: Option<Point> = r.cast(b);

                    match point {
                        Some(p) => Line::new([0.5, 0.5, 0.5, 1.0], 1.0).draw(
                            [r.x(), r.y(), p.x(), p.y()],
                            &c.draw_state,
                            c.transform,
                            gl,
                        ),
                        None => {}
                    }
                }
            }

            // Clear the screen.
            clear(BLACK, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}

    fn set_mouse_pos(&mut self, args: &[f64; 2]) {
        self.mouse_pos = *args;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("raycasting", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new app and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        mouse_pos: [0.0; 2],
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.mouse_cursor_args() {
            app.set_mouse_pos(&args);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

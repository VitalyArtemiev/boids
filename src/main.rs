#![feature(derive_default_enum)]

mod app;
mod boids;
mod container;
mod ops;
mod player;
mod world;

use crate::app::App;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::Event;

//static arr : Array2D<i32> = Array2D::filled_with(0, 5, 5);

///docstring
/// assert_equals executes automatically as integration test in docs

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("boids", [1000, 1000])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e.clone() {
            Event::Input(input, _) => app.handle_input(input),
            Event::Loop(l) => {}
            Event::Custom(a, b, c) => {}
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

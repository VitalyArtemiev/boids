extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use self::piston::{Input, Motion, ButtonArgs, Button, MouseButton};


use crate::boids::{Boid, BoidVec};
use self::graphics::math::Vec2d;
use std::borrow::Borrow;
use crate::ops::Vec2f;

pub struct App {
    pub(crate) gl: GlGraphics, // OpenGL drawing backend.
    pub(crate) rotation: f64,  // Rotation for the square.

    boids: BoidVec,
    attractor:  Vec2f,
    mousePos: Vec2f,
    screenOffset: Vec2f
}

const BOID_NUM: usize = 20;

impl App {
    pub(crate) fn new(gl: OpenGL) -> Self {
        App {
            gl: GlGraphics::new(gl),
            rotation: 0.0,
            boids: BoidVec::random(BOID_NUM),
            attractor:  Vec2f {x: 0.,y: 0.},
            mousePos: Vec2f {x: 0.,y: 0.},
            screenOffset: Vec2f {x: -512.,y: -512.}        }
    }

    pub(crate) fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let cursor = ellipse::circle(0.,0., 12.);
        let square = rectangle::square(0.0, 0.0, 24.0);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.screenOffset.x = -x;
        self.screenOffset.y = -y;

        let c = self.gl.draw_begin(args.viewport());

        // Clear the screen.
        clear(GREEN, &mut self.gl);

        for boid in self.boids.iter() {
            let transform = c
            .transform
            .trans(x + boid.pos.x, y + boid.pos.y)
            .rot_rad(* boid.r)
            .trans(-25.0, -25.0);

        // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, &mut self.gl);
        }

        let transform = c
            .transform
            .trans(x + self.attractor.x, y + self.attractor.y)
            .trans(-25.0, -25.0);

        ellipse(BLUE, cursor, transform, &mut self.gl);
        
        self.gl.draw_end();

        //self.gl.draw(args.viewport(), |c, gl| {
        //    
        //});
    }

    pub(crate) fn handle_input(&mut self, input_event: Input) {
        match input_event {
            Input::Button(a) => {
                match a.button {
                    Button::Keyboard(_) => {}
                    Button::Mouse(mb) => {
                        match mb {
                            MouseButton::Unknown => {}
                            MouseButton::Left => {self.attractor = self.mousePos + self.screenOffset}
                            MouseButton::Right => {}
                            MouseButton::Middle => {}
                            MouseButton::X1 => {}
                            MouseButton::X2 => {}
                            MouseButton::Button6 => {}
                            MouseButton::Button7 => {}
                            MouseButton::Button8 => {}
                        }
                    }
                    Button::Controller(_) => {}
                    Button::Hat(_) => {}
                }
                //println!("code: ", a.scancode)
            }
            Input::Move(m) => {
                match m {
                    Motion::MouseCursor(pos) => {self.mousePos = Vec2f{x: pos[0], y:pos[1]}}
                    Motion::MouseRelative(_) => {}
                    Motion::MouseScroll(_) => {}
                    Motion::ControllerAxis(_) => {}
                    Motion::Touch(_) => {}
                }
            }
            Input::Text(_) => {}
            Input::Resize(_) => {}
            Input::Focus(_) => {}
            Input::Cursor(_) => {}
            Input::FileDrag(_) => {}
            Input::Close(_) => {}
        }
    }

    pub(crate) fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.boids.upd_position(args.dt, self.attractor);
        self.rotation += 2.0 * args.dt;
    }
}
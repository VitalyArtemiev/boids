extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use self::piston::{Button, ButtonState, Input, Motion, MouseButton};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};

use crate::boids::{Boid, BoidVec};
use crate::ops::Vec2f;
use crate::player::PlayerState;
use crate::world::World;

pub struct App {
    pub(crate) gl: GlGraphics, // OpenGL drawing backend.

    player: PlayerState,
    mouse_pos: Vec2f,
    screen_offset: Vec2f,
    world: World,
}

const BOID_SIZE: f64 = 24.;
const CURSOR_SIZE: f64 = 12.;
pub const CLICK_PRECISION: f64 = 40000.;

impl App {
    pub(crate) fn new(gl: OpenGL) -> Self {
        App {
            gl: GlGraphics::new(gl),
            player: Default::default(),
            mouse_pos: Default::default(),
            screen_offset: Vec2f { x: -512., y: -512. },
            world: World::single_container(),
        }
    }

    pub(crate) fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let p = &self.player;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const TRANSP_BLUE: [f32; 4] = [0.0, 0.0, 1.0, 0.2];

        let cursor = ellipse::circle(0., 0., CURSOR_SIZE);
        let square = rectangle::square(0.0, 0.0, BOID_SIZE);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.screen_offset.x = -x;
        self.screen_offset.y = -y;

        let c = self.gl.draw_begin(args.viewport());

        // Clear the screen.
        clear(GREEN, &mut self.gl);

        for group in &mut self.world.groups {
            for boid in group.ent.iter() {
                let transform = c
                    .transform
                    .trans(x + boid.pos.x, y + boid.pos.y)
                    .rot_rad(*boid.r)
                    .trans(-BOID_SIZE / 2., -BOID_SIZE / 2.);

                // Draw a box rotating around the middle of the screen.
                rectangle(*boid.color, square, transform, &mut self.gl);
            }
        }

        let transform = c
            .transform
            .trans(x + p.l1.x, y + p.l1.y)
            .trans(-CURSOR_SIZE / 2., -CURSOR_SIZE / 2.);

        ellipse(BLUE, cursor, transform, &mut self.gl);

        let transform = c
            .transform
            .trans(x + p.l2.x, y + p.l2.y)
            .trans(-CURSOR_SIZE / 2., -CURSOR_SIZE / 2.);

        ellipse(BLUE, cursor, transform, &mut self.gl);

        let transform = c
            .transform
            .trans(x, y)
            .trans(-CURSOR_SIZE / 2., -CURSOR_SIZE / 2.);

        line_from_to(BLUE, 3., p.l1, p.l2, transform, &mut self.gl);

        let transform = c
            .transform
            .trans(x + p.r1.x, y + p.r1.y)
            .trans(-CURSOR_SIZE / 2., -CURSOR_SIZE / 2.);

        ellipse(RED, cursor, transform, &mut self.gl);

        let transform = c
            .transform
            .trans(x + p.r2.x, y + p.r2.y)
            .trans(-CURSOR_SIZE / 2., -CURSOR_SIZE / 2.);

        ellipse(RED, cursor, transform, &mut self.gl);

        let transform = c
            .transform
            .trans(x, y)
            .trans(-CURSOR_SIZE / 2., -CURSOR_SIZE / 2.);

        line_from_to(RED, 3., p.r1, p.r2, transform, &mut self.gl);

        let sel_rect = rectangle::rectangle_by_corners(p.l1.x, p.l1.y, p.l2.x, p.l2.y);

        rectangle(TRANSP_BLUE, sel_rect, transform, &mut self.gl);

        self.gl.draw_end();

        //self.gl.draw(args.viewport(), |c, gl| {
        //
        //});
    }

    pub(crate) fn handle_input(&mut self, input_event: Input) {
        let p = &mut self.player;

        p.l_click = false;
        p.r_click = false;

        match input_event {
            Input::Button(a) => match a.button {
                Button::Keyboard(_) => {}
                Button::Mouse(mb) => match mb {
                    MouseButton::Unknown => {}
                    MouseButton::Left => match a.state {
                        ButtonState::Press => {
                            p.l_pressed = true;
                            p.l1 = self.mouse_pos + self.screen_offset;
                            p.l2 = self.mouse_pos + self.screen_offset
                        }
                        ButtonState::Release => {
                            p.l_pressed = false;
                            p.l_click = true;
                        }
                    },
                    MouseButton::Right => match a.state {
                        ButtonState::Press => {
                            p.r_pressed = true;
                            p.r1 = self.mouse_pos + self.screen_offset;
                            p.r2 = self.mouse_pos + self.screen_offset
                        }
                        ButtonState::Release => {
                            p.r_pressed = false;
                            p.r_click = true;
                        }
                    },
                    MouseButton::Middle => {}
                    MouseButton::X1 => {}
                    MouseButton::X2 => {}
                    MouseButton::Button6 => {}
                    MouseButton::Button7 => {}
                    MouseButton::Button8 => {}
                },
                Button::Controller(_) => {}
                Button::Hat(_) => {}
            },
            Input::Move(m) => match m {
                Motion::MouseCursor(pos) => {
                    self.mouse_pos = Vec2f {
                        x: pos[0],
                        y: pos[1],
                    };
                    if p.l_pressed {
                        p.l2 = self.mouse_pos + self.screen_offset;
                    }
                    if p.r_pressed {
                        p.r2 = self.mouse_pos + self.screen_offset;
                    }
                }
                Motion::MouseRelative(_) => {}
                Motion::MouseScroll(_) => {}
                Motion::ControllerAxis(_) => {}
                Motion::Touch(_) => {}
            },
            Input::Text(_) => {}
            Input::Resize(_) => {}
            Input::Focus(_) => {}
            Input::Cursor(_) => {}
            Input::FileDrag(_) => {}
            Input::Close(_) => {}
        }

        p.update_player_action(&self.world);
    }

    pub(crate) fn update(&mut self, args: &UpdateArgs) {
        for group in &mut self.world.groups {
            if self.player.selected.contains(&group.id) {
                group.assign_goals(self.player.action);
            }

            group.process_boids(args.dt, &self.player);
        }
    }
}

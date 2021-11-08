extern crate lazy_static;
use crate::boids::BoidRef;
use crate::container::{Container, Goal};
use crate::ops::Vec2f;
use graphics::types::Rectangle;
use graphics::*;
use lazy_static::lazy_static;
use std::sync::atomic::AtomicPtr;

pub trait Drawable {
    fn draw<G>(&self, c: Context, offset: Vec2f, g: &mut G)
    where
        G: Graphics;
}

const BOID_SIZE: f64 = 24.;

lazy_static! {
    static ref boid_square: Rectangle = rectangle::square(0.0, 0.0, BOID_SIZE);
}

impl Drawable for BoidRef<'_> {
    fn draw<G>(&self, c: Context, offset: Vec2f, g: &mut G)
    where
        G: Graphics,
    {
        println!("draw");
        let transform = c
            .transform
            .trans( - offset.x + self.pos.x, - offset.y + self.pos.y)
            .rot_rad(*self.r)
            .trans(-BOID_SIZE / 2., -BOID_SIZE / 2.);

        rectangle(*self.color, *boid_square, transform, g);
    }
}

const CURSOR_SIZE: f64 = 12.;

const GREEN: [f32; 4] = [0.1, 0.5, 0.1, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const TRANSP_BLUE: [f32; 4] = [0.0, 0.0, 1.0, 0.2];
const TRANSP_ORANGE: [f32; 4] = [0.5, 0.1, 0.0, 0.1];
const TRANSP_RED: [f32; 4] = [0.9, 0.1, 0.0, 0.1];

impl Drawable for Container {
    fn draw<G>(&self, c: Context, offset: Vec2f, g: &mut G)
    where
        G: Graphics,
    {
        for boid in self.ent.iter() {
            //let b: Boid = boid;
            boid.draw(c, offset, g);
        }

        let transform = c.transform.trans( - offset.x + self.center.x, - offset.y + self.center.y);

        let self_area = ellipse::circle(0., 0., self.radius);

        /*if self.player.selected.contains(&self.id) {
            ellipse(TRANSP_RED, self_area, transform, g);
        } else {
            ellipse(TRANSP_ORANGE, self_area, transform, g);
        }*/

        if let Some(goal) = self.goals.front() {
            let transform = c
                .transform
                .trans( - offset.x, - offset.y)
                .trans(-CURSOR_SIZE / 2., -CURSOR_SIZE / 2.);

            match goal {
                Goal::Idle(_) => {}
                Goal::Hold => {}
                Goal::Move(_, _) => {}
                Goal::Column(_) => {}
                Goal::Front(p1, p2, d) => {
                    line_from_to(TRANSP_RED, 5., *p1, *p2, transform, g);
                    line_from_to(
                        TRANSP_RED,
                        5.,
                        self.center,
                        self.center + *d,
                        transform,
                        g,
                    );
                }
            }
        }
    }
}

/*const CURSOR_SIZE: f64 = 12.;
impl Drawable for
*/

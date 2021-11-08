#![feature(test)]
#![feature(derive_default_enum)]
extern crate test;
use boids::boids::*;
use boids::container::{Container, ContainerState};
use boids::ops::Vec2f;
use boids::player::PlayerState;
use test::Bencher;

const BOID_NUM: usize = 1023;

#[bench]
fn bench_boids(b: &mut Bencher) {
    let mut c = Container::new(Vec2f::default() ,BOID_NUM);

    let p = PlayerState::default();

    let dt = 0.01;

    b.iter(|| {
        c.process_boids(dt, &p);
    });
}

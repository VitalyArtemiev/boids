#![feature(test)]
#![feature(derive_default_enum)]
extern crate test;
use test::Bencher;
use boids::container::{Container, ContainerState};
use boids::boids::*;
use boids::player::PlayerState;

const BOID_NUM: usize = 2000;

#[bench]
fn bench_xor_1000_ints(b: &mut Bencher) {
    b.iter(|| {
        (0..1000).fold(0, |old, new| old ^ new);
    });
}

#[bench]
fn bench_boids(b: &mut Bencher) {
    let mut c = Container {
        id: 0,
        center: Default::default(),
        radius: 0.0,
        ent: BoidVec::random(BOID_NUM),
        goals: vec![],
        state: ContainerState::Cold
    };

    let p = PlayerState::default();

    let dt = 0.01;

    b.iter(|| {
        c.process_boids(dt, &p);
    });
}



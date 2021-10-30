#![feature(test)]
#![feature(derive_default_enum)]
#![feature(bench_black_box)]
extern crate test;
use test::Bencher;
use boids::container::{Container, ContainerState};
use boids::boids::*;
use boids::ops::{Vec2, Vec2f};
use boids::player::PlayerState;
use core::hint::black_box;

const ENT_NUM: usize = 2000;

#[bench]
fn bench_struct_clone(b: &mut Bencher) {
    let mut mystruct = Vec2f::default();

    b.iter(|| {
        for i in 0..ENT_NUM {
            let new_struct = mystruct.clone();
            mystruct.x = black_box(new_struct.x);
        }
    });
}

#[bench]
fn bench_arr_clone(b: &mut Bencher) {
    let mut arr = [0.; 2];

    b.iter(|| {
        for i in 0..ENT_NUM {
            let new_arr = arr.clone();
            arr[0] = black_box(new_arr[0]);
        }
    });
}

#[bench]
fn bench_tuple_clone(b: &mut Bencher) {
    let mut tuple = (0., 0.);

    b.iter(|| {
        for i in 0..ENT_NUM {
            let new_tuple = tuple.clone();
            tuple.0 = black_box(new_tuple.0);
        }
    });
}

#[bench]
fn bench_structs_clone(b: &mut Bencher) {
    let mut structs = [Vec2f::default(); ENT_NUM];

    b.iter(|| {
        for i in 0..ENT_NUM {
            let new_structs = structs.clone();
            structs[i] = black_box(new_structs[0]);
        }
    });
}

#[bench]
fn bench_arrays_clone(b: &mut Bencher) {
    let mut structs = [[0.; 2]; ENT_NUM];

    b.iter(|| {
        for i in 0..ENT_NUM {
            let new_structs = structs.clone();
            structs[i] = black_box(new_structs[0]);
        }
    });
}

#[bench]
fn bench_tuples_clone(b: &mut Bencher) {
    let mut structs = [(0., 0.); ENT_NUM];

    b.iter(|| {
        for i in 0..ENT_NUM {
            let new_structs = structs.clone();
            structs[i] = black_box(new_structs[0]);
        }
    });
}

#[bench]
fn bench_struct_vec_clone(b: &mut Bencher) {
    let mut structs = vec![Vec2f::default(); ENT_NUM];

    b.iter(|| {
        for i in 0..ENT_NUM {
            let new_structs = structs.clone();
            structs[i] = black_box(new_structs[0]);
        }
    });
}

#[bench]
fn bench_array_vec_clone(b: &mut Bencher) {
    let mut structs = vec![[0.; 2]; ENT_NUM];

    b.iter(|| {
        for i in 0..ENT_NUM {
            let new_structs = structs.clone();
            structs[i] = black_box(new_structs[0]);
        }
    });
}

#[bench]
fn bench_tuple_vec_clone(b: &mut Bencher) {
    let mut structs = vec![(0., 0.); ENT_NUM];

    b.iter(|| {
        for i in 0..ENT_NUM {
            let new_structs = structs.clone();
            structs[i] = black_box(new_structs[0]);
        }
    });
}

//results: static arrays are 1000x faster than vecs, structs are 20% faster in array[struct] than arrays or tuples
/*

test bench_arr_clone        ... bench:       7,272 ns/iter (+/- 90)
test bench_struct_clone     ... bench:       4,379 ns/iter (+/- 15)
test bench_tuple_clone      ... bench:       4,332 ns/iter (+/- 82)

test bench_arrays_clone     ... bench:       1,263 ns/iter (+/- 31)
test bench_structs_clone    ... bench:       1,009 ns/iter (+/- 571)
test bench_tuples_clone     ... bench:       1,267 ns/iter (+/- 15)

test bench_array_vec_clone  ... bench:   1,421,297 ns/iter (+/- 54,761)
test bench_struct_vec_clone ... bench:   1,422,417 ns/iter (+/- 110,886)
test bench_tuple_vec_clone  ... bench:   1,412,190 ns/iter (+/- 62,980)

 */






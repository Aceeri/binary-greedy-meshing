
use std::collections::BTreeSet;

use binary_greedy_meshing as bgm;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
const CHUNK_SIZE: usize = 32;
const CS_H: usize = CHUNK_SIZE/2;
const SIZE: usize = 16;
const SIZE2: usize = SIZE.pow(2);

fn voxel_buffer() -> Box<[u16; bgm::Mesher::<CHUNK_SIZE>::CS_P3]> {
    let mut voxels = Box::new([0; bgm::Mesher::<CHUNK_SIZE>::CS_P3]);
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                voxels[bgm::pad_linearize::<CHUNK_SIZE>(x, y, z)] = sphere(x, y, z);
            }
        }
    }
    voxels
}

fn sphere(x: usize, y: usize, z: usize) -> u16 {
    if (x - CS_H).pow(2) + (y - CS_H).pow(2) + (z - CS_H).pow(2) < SIZE2 {
        1
    } else {
        0
    }
}

fn bench_opaque_32(c: &mut Criterion) {
    let voxels = voxel_buffer();
    let mut mesher = bgm::Mesher::<CHUNK_SIZE>::new();
    c.bench_function("bench_opaque", |b| b.iter(|| {
        mesher.clear();
        mesher.mesh(
            black_box(voxels.as_slice()), black_box(&BTreeSet::default())
        );
        black_box(&mut mesher);
    }));
}

fn bench_transparent_32(c: &mut Criterion) {
    let voxels = voxel_buffer();
    let mut transparents = BTreeSet::default();
    transparents.insert(2);
    transparents.insert(3);
    let mut mesher = bgm::Mesher::<CHUNK_SIZE>::new();
    c.bench_function("bench_transparent", |b| b.iter(|| {
        mesher.clear();
        mesher.mesh(
            black_box(voxels.as_slice()), black_box(&BTreeSet::default())
        );
        black_box(&mut mesher);
    }));
}

criterion_group!(
    mesh_32, 
    bench_opaque_32, 
    bench_transparent_32
);
criterion_main!(mesh_32);
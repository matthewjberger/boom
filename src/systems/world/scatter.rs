//! Streamed natural scatter for the overworld: deterministic per-chunk trees,
//! rocks, and bushes seated on the terrain and drawn as instanced batches (one
//! draw call per prop type per chunk). Chunks load in a radius around the player
//! and unload past a larger radius, so walking the hills stays populated without
//! ever holding the whole world in memory. Everything is derived from chunk
//! coordinates, so a chunk regenerates identically after it is unloaded.

use std::collections::HashMap;

use crate::systems::world::textures::{MAT_FOLIAGE, MAT_FOLIAGE_WARM, MAT_ROCK, MAT_TRUNK};
use nalgebra_glm::{Vec3, quat_angle_axis, vec3};
use nightshade::prelude::*;

const CHUNK: f32 = 40.0;
const LOAD_RADIUS: i32 = 6;
const UNLOAD_RADIUS: i32 = 8;
const TOWN_CLEAR: f32 = 78.0;
const MAX_TREES: f32 = 16.0;
const BUDGET_PER_FRAME: usize = 6;
const TAU: f32 = std::f32::consts::TAU;

#[derive(Default)]
pub struct ScatterState {
    chunks: HashMap<(i32, i32), Vec<Entity>>,
    last_chunk: (i32, i32),
    primed: bool,
}

pub fn update(state: &mut ScatterState, world: &mut World, camera: Vec3) {
    let camera_chunk = (
        (camera.x / CHUNK).floor() as i32,
        (camera.z / CHUNK).floor() as i32,
    );

    if camera_chunk != state.last_chunk || !state.primed {
        state.last_chunk = camera_chunk;
        state.primed = true;
        unload_distant(state, world, camera_chunk);
    }

    load_nearby(state, world, camera_chunk);
}

/// Despawn and forget every scatter chunk (leaving the overworld or a cell).
pub fn clear(state: &mut ScatterState, world: &mut World) {
    for (_, entities) in state.chunks.drain() {
        for entity in entities {
            despawn_recursive_immediate(world, entity);
        }
    }
    state.primed = false;
}

fn unload_distant(state: &mut ScatterState, world: &mut World, camera_chunk: (i32, i32)) {
    let stale: Vec<(i32, i32)> = state
        .chunks
        .keys()
        .copied()
        .filter(|coord| chebyshev(*coord, camera_chunk) > UNLOAD_RADIUS)
        .collect();
    for coord in stale {
        if let Some(entities) = state.chunks.remove(&coord) {
            for entity in entities {
                despawn_recursive_immediate(world, entity);
            }
        }
    }
}

fn load_nearby(state: &mut ScatterState, world: &mut World, camera_chunk: (i32, i32)) {
    let mut candidates: Vec<(i32, i32)> = Vec::new();
    for offset_z in -LOAD_RADIUS..=LOAD_RADIUS {
        for offset_x in -LOAD_RADIUS..=LOAD_RADIUS {
            let coord = (camera_chunk.0 + offset_x, camera_chunk.1 + offset_z);
            if !state.chunks.contains_key(&coord) {
                candidates.push(coord);
            }
        }
    }
    candidates.sort_by_key(|coord| chebyshev(*coord, camera_chunk));

    let mut spawned = 0;
    for coord in candidates {
        if spawned >= BUDGET_PER_FRAME {
            break;
        }
        // Defer until the terrain heightfield for this chunk has streamed in, so
        // props sit on the ground rather than at y=0.
        let center_x = coord.0 as f32 * CHUNK + CHUNK * 0.5;
        let center_z = coord.1 as f32 * CHUNK + CHUNK * 0.5;
        if height(world, center_x, center_z).is_none() {
            continue;
        }
        let entities = generate_chunk(world, coord);
        state.chunks.insert(coord, entities);
        spawned += 1;
    }
}

fn generate_chunk(world: &mut World, coord: (i32, i32)) -> Vec<Entity> {
    let base_x = coord.0 as f32 * CHUNK;
    let base_z = coord.1 as f32 * CHUNK;
    let forest = value_noise(coord.0 as f32 * 0.13, coord.1 as f32 * 0.13, 0x51);
    let rockiness = value_noise(
        coord.0 as f32 * 0.19 + 40.0,
        coord.1 as f32 * 0.19 - 17.0,
        0x77,
    );
    let mut rng = ChunkRng::new(coord.0, coord.1);
    let mut batch = InstanceBatch::default();

    let tree_target = (forest * forest * MAX_TREES) as i32;
    for _ in 0..tree_target {
        let x = base_x + rng.f01() * CHUNK;
        let z = base_z + rng.f01() * CHUNK;
        if x * x + z * z < TOWN_CLEAR * TOWN_CLEAR {
            continue;
        }
        if let Some(ground) = height(world, x, z) {
            add_tree(&mut batch, &mut rng, vec3(x, ground, z));
        }
    }

    let rock_target = (rockiness * 5.0) as i32;
    for _ in 0..rock_target {
        let x = base_x + rng.f01() * CHUNK;
        let z = base_z + rng.f01() * CHUNK;
        if x * x + z * z < TOWN_CLEAR * TOWN_CLEAR {
            continue;
        }
        if let Some(ground) = height(world, x, z) {
            add_rock(&mut batch, &mut rng, vec3(x, ground, z));
        }
    }

    let bush_target = (forest * 6.0) as i32;
    for _ in 0..bush_target {
        let x = base_x + rng.f01() * CHUNK;
        let z = base_z + rng.f01() * CHUNK;
        if x * x + z * z < TOWN_CLEAR * TOWN_CLEAR {
            continue;
        }
        if let Some(ground) = height(world, x, z) {
            add_bush(&mut batch, &mut rng, vec3(x, ground, z));
        }
    }

    batch.instantiate(world)
}

fn add_tree(batch: &mut InstanceBatch, rng: &mut ChunkRng, base: Vec3) {
    let conifer = rng.f01() < 0.6;
    let trunk_height = rng.range(2.4, 5.2);
    let trunk_radius = rng.range(0.5, 0.8);
    let yaw = rng.range(0.0, TAU);
    let rotation = quat_angle_axis(yaw, &vec3(0.0, 1.0, 0.0));
    batch.push(
        "Cylinder",
        MAT_TRUNK,
        InstanceTransform::new(
            base + vec3(0.0, trunk_height * 0.5, 0.0),
            rotation,
            vec3(trunk_radius, trunk_height, trunk_radius),
        ),
    );
    let foliage = if rng.f01() < 0.22 {
        MAT_FOLIAGE_WARM
    } else {
        MAT_FOLIAGE
    };
    if conifer {
        let base_radius = rng.range(3.2, 4.8);
        let tier_height = rng.range(2.2, 3.0);
        let mut y = trunk_height * 0.75;
        for tier in 0..3 {
            let radius = base_radius * (1.0 - tier as f32 * 0.24);
            batch.push(
                "Cone",
                foliage,
                InstanceTransform::new(
                    base + vec3(0.0, y + tier_height * 0.5, 0.0),
                    rotation,
                    vec3(radius, tier_height, radius),
                ),
            );
            y += tier_height * 0.6;
        }
    } else {
        let canopy_radius = rng.range(2.2, 3.4);
        let canopy_height = rng.range(2.8, 4.2);
        batch.push(
            "Sphere",
            foliage,
            InstanceTransform::new(
                base + vec3(0.0, trunk_height + canopy_height * 0.3, 0.0),
                rotation,
                vec3(canopy_radius, canopy_height, canopy_radius),
            ),
        );
    }
}

fn add_rock(batch: &mut InstanceBatch, rng: &mut ChunkRng, base: Vec3) {
    let size = rng.range(0.8, 2.6);
    let rotation = quat_angle_axis(rng.range(0.0, TAU), &vec3(0.0, 1.0, 0.0))
        * quat_angle_axis(rng.range(-0.5, 0.5), &vec3(1.0, 0.0, 0.0));
    batch.push(
        "Cube",
        MAT_ROCK,
        InstanceTransform::new(
            base + vec3(0.0, size * 0.3, 0.0),
            rotation,
            vec3(size, size * rng.range(0.55, 0.95), size),
        ),
    );
}

fn add_bush(batch: &mut InstanceBatch, rng: &mut ChunkRng, base: Vec3) {
    let radius = rng.range(0.7, 1.5);
    batch.push(
        "Sphere",
        MAT_FOLIAGE,
        InstanceTransform::from_translation_scale(
            base + vec3(0.0, radius * 0.5, 0.0),
            vec3(radius, radius * 0.8, radius),
        ),
    );
}

fn height(world: &World, x: f32, z: f32) -> Option<f32> {
    world.resources.terrain_render.sample_height(x, z)
}

#[derive(Default)]
struct InstanceBatch {
    groups: Vec<((&'static str, &'static str), Vec<InstanceTransform>)>,
    index: HashMap<(&'static str, &'static str), usize>,
}

impl InstanceBatch {
    fn push(&mut self, mesh: &'static str, material: &'static str, transform: InstanceTransform) {
        let key = (mesh, material);
        if let Some(&group) = self.index.get(&key) {
            self.groups[group].1.push(transform);
        } else {
            let group = self.groups.len();
            self.index.insert(key, group);
            self.groups.push((key, vec![transform]));
        }
    }

    fn instantiate(self, world: &mut World) -> Vec<Entity> {
        let mut entities = Vec::new();
        for ((mesh, material), instances) in self.groups {
            if instances.is_empty() {
                continue;
            }
            entities.push(spawn_instanced_mesh_with_material(
                world, mesh, instances, material,
            ));
        }
        if !entities.is_empty() {
            world
                .resources
                .mesh_render_state
                .mark_instanced_meshes_changed();
        }
        entities
    }
}

fn chebyshev(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs().max((a.1 - b.1).abs())
}

/// A small counter-based RNG seeded from chunk coordinates for deterministic,
/// storage-free placement.
struct ChunkRng(u64);

impl ChunkRng {
    fn new(chunk_x: i32, chunk_z: i32) -> Self {
        let seed = (chunk_x as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
            ^ (chunk_z as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f)
            ^ 0x0B12_5709_5147_4E59;
        Self(seed)
    }

    fn next(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 33) as u32
    }

    fn f01(&mut self) -> f32 {
        (self.next() & 0x00ff_ffff) as f32 / 0x0100_0000 as f32
    }

    fn range(&mut self, low: f32, high: f32) -> f32 {
        low + (high - low) * self.f01()
    }
}

fn hash2(x: i32, z: i32, seed: u32) -> u32 {
    let mut h = (x as u32).wrapping_mul(0x8da6_b343)
        ^ (z as u32).wrapping_mul(0xd816_3841)
        ^ seed.wrapping_mul(0x1b56_c4e9);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2c1b_3c6d);
    h ^= h >> 12;
    h = h.wrapping_mul(0x297a_2d39);
    h ^= h >> 15;
    h
}

fn hash01(x: i32, z: i32, seed: u32) -> f32 {
    (hash2(x, z, seed) & 0x00ff_ffff) as f32 / 0x0100_0000 as f32
}

/// Smooth value noise in roughly `[0, 1]` for coherent biome fields.
fn value_noise(x: f32, z: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let z0 = z.floor() as i32;
    let fx = x - x0 as f32;
    let fz = z - z0 as f32;
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sz = fz * fz * (3.0 - 2.0 * fz);
    let n00 = hash01(x0, z0, seed);
    let n10 = hash01(x0 + 1, z0, seed);
    let n01 = hash01(x0, z0 + 1, seed);
    let n11 = hash01(x0 + 1, z0 + 1, seed);
    let a = n00 + (n10 - n00) * sx;
    let b = n01 + (n11 - n01) * sx;
    a + (b - a) * sz
}

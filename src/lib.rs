//! BRIMSTONE — a boomer-shooter built on the nightshade engine.
//!
//! Movement is the soul of it: Quake-style strafe-jumping, a dash, jump pads,
//! and wallrunning with wall-jumps. Three weapons (shotgun, nailgun, splash
//! rocket with rocket-jumps), six enemy archetypes (imp, swarmer, caster,
//! brute, flying gargoyle, flying sentinel) plus elites and a warlord boss,
//! and a push-forward combo economy that pays out overheal and ammo.
//!
//! Three ways to play: a six-mission Story campaign with objectives
//! (exterminate / reach the gate / kill the warlord / recover the keycard) and
//! text cutscenes; an endless Arcade level cycle; and an in-game level editor
//! whose creations are playable on the spot.
//!
//! Architecture follows the nightshade sandbox style: a state shell forwarding
//! to system functions over a user-side ECS world.

mod adventure;
mod art;
mod campaign;
mod content;
mod ecs;
mod settings;
mod state;
mod systems;
mod theme;
mod tuning;

pub use state::Brimstone;

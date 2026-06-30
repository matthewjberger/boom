//! Hand-authored level definitions. Each level is a distinct layout with its
//! own geometry, sky, enemy roster, player spawn, and exit gate. The game
//! advances through them and loops with scaling difficulty.

use nightshade::prelude::Atmosphere;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum BlockKind {
    #[default]
    Wall,
    Pillar,
    Cover,
    Choke,
    Monument,
    Platform,
}

impl BlockKind {
    pub const ALL: [BlockKind; 6] = [
        BlockKind::Wall,
        BlockKind::Pillar,
        BlockKind::Platform,
        BlockKind::Cover,
        BlockKind::Choke,
        BlockKind::Monument,
    ];

    pub fn label(self) -> &'static str {
        match self {
            BlockKind::Wall => "WALL",
            BlockKind::Pillar => "PILLAR",
            BlockKind::Cover => "COVER",
            BlockKind::Choke => "CHOKE",
            BlockKind::Monument => "MONUMENT",
            BlockKind::Platform => "PLATFORM",
        }
    }

    pub fn code(self) -> u8 {
        match self {
            BlockKind::Wall => 0,
            BlockKind::Pillar => 1,
            BlockKind::Cover => 2,
            BlockKind::Choke => 3,
            BlockKind::Monument => 4,
            BlockKind::Platform => 5,
        }
    }

    pub fn from_code(code: u8) -> BlockKind {
        match code {
            1 => BlockKind::Pillar,
            2 => BlockKind::Cover,
            3 => BlockKind::Choke,
            4 => BlockKind::Monument,
            5 => BlockKind::Platform,
            _ => BlockKind::Wall,
        }
    }
}

/// (cx, cy, cz, sx, sy, sz, kind)
pub type BlockSpec = (f32, f32, f32, f32, f32, f32, BlockKind);
/// (x, z, [r, g, b])
pub type BeaconSpec = (f32, f32, [f32; 3]);
/// (cx, cy, cz, sx, sy, sz, pitch_radians, yaw_radians) — a tilted slab you can
/// walk up. Keep pitch gentle so the character controller climbs it.
pub type RampSpec = (f32, f32, f32, f32, f32, f32, f32, f32);

#[derive(Clone, Copy, Default)]
pub struct Roster {
    pub imps: u32,
    pub swarmers: u32,
    pub casters: u32,
    pub brutes: u32,
    pub gargoyles: u32,
    pub sentinels: u32,
}

/// An owned, mutable level — what the in-game editor builds and what custom
/// play sessions run from. Mirrors [`Level`] but with growable collections.
#[derive(Clone)]
pub struct LevelData {
    pub name: String,
    pub atmosphere_index: u8,
    pub fog: [f32; 3],
    pub spawn: [f32; 3],
    pub exit: [f32; 2],
    pub blocks: Vec<BlockSpec>,
    pub pads: Vec<(f32, f32)>,
    pub spawn_points: Vec<(f32, f32)>,
    pub roster: Roster,
}

impl Default for LevelData {
    fn default() -> Self {
        Self {
            name: "CUSTOM".to_string(),
            atmosphere_index: 0,
            fog: [0.05, 0.03, 0.10],
            spawn: [0.0, 1.2, 16.0],
            exit: [0.0, -16.5],
            blocks: Vec::new(),
            pads: Vec::new(),
            spawn_points: Vec::new(),
            roster: Roster {
                imps: 6,
                swarmers: 5,
                casters: 2,
                brutes: 0,
                gargoyles: 1,
                sentinels: 0,
            },
        }
    }
}

pub fn atmosphere_for(index: u8) -> Atmosphere {
    match index % 3 {
        1 => Atmosphere::Sunset,
        2 => Atmosphere::Space,
        _ => Atmosphere::Nebula,
    }
}

pub struct Level {
    pub name: &'static str,
    pub atmosphere: Atmosphere,
    pub fog: [f32; 3],
    pub spawn: [f32; 3],
    pub exit: [f32; 2],
    pub blocks: &'static [BlockSpec],
    pub ramps: &'static [RampSpec],
    pub beacons: &'static [BeaconSpec],
    pub spawn_points: &'static [(f32, f32)],
    pub pads: &'static [(f32, f32)],
    pub roster: Roster,
}

pub fn count() -> usize {
    LEVELS.len()
}

pub fn level(index: usize) -> &'static Level {
    &LEVELS[index % LEVELS.len()]
}

use BlockKind::{Choke, Cover, Monument, Pillar, Platform, Wall};

const L1_BLOCKS: &[BlockSpec] = &[
    (0.0, 3.5, 0.0, 3.0, 7.0, 3.0, Monument),
    (9.0, 2.0, 6.0, 1.8, 4.0, 1.8, Pillar),
    (-8.0, 2.3, 7.5, 1.8, 4.6, 1.8, Pillar),
    (-10.5, 1.6, -6.0, 1.8, 3.2, 1.8, Pillar),
    (7.5, 1.6, -9.5, 1.8, 3.2, 1.8, Pillar),
    (4.0, 0.45, 9.5, 3.4, 0.9, 1.4, Cover),
    (-4.5, 0.45, -8.5, 3.4, 0.9, 1.4, Cover),
    // Raised perch with a step up, reachable by jump or the nearby pad.
    (11.5, 1.1, -4.0, 4.5, 2.2, 4.5, Platform),
    (8.4, 0.5, -2.0, 2.0, 1.0, 2.0, Platform),
    (-11.5, 1.4, 2.5, 4.5, 2.8, 4.5, Platform),
    (-8.4, 0.6, 1.0, 2.0, 1.2, 2.0, Platform),
];
const L1_BEACONS: &[BeaconSpec] = &[
    (5.0, 5.0, [0.2, 1.5, 1.8]),
    (-5.0, 5.0, [1.6, 0.3, 1.5]),
    (5.0, -5.0, [1.7, 0.8, 0.2]),
    (-5.0, -5.0, [0.3, 1.6, 0.5]),
];
const L1_SPAWNS: &[(f32, f32)] = &[
    (0.0, -16.0),
    (14.0, -8.0),
    (-14.0, -8.0),
    (14.0, 8.0),
    (-14.0, 8.0),
];
const L1_PADS: &[(f32, f32)] = &[(13.5, -1.0), (-13.5, 5.5)];

const L2_BLOCKS: &[BlockSpec] = &[
    (5.5, 1.75, 0.0, 1.0, 3.5, 22.0, Wall),
    (-5.5, 1.75, 0.0, 1.0, 3.5, 22.0, Wall),
    (11.5, 1.75, 7.0, 7.0, 3.5, 1.0, Wall),
    (-11.5, 1.75, 7.0, 7.0, 3.5, 1.0, Wall),
    (11.5, 1.75, -7.0, 7.0, 3.5, 1.0, Wall),
    (-11.5, 1.75, -7.0, 7.0, 3.5, 1.0, Wall),
    (0.0, 0.5, 6.0, 2.2, 1.0, 1.0, Choke),
    (0.0, 0.5, -6.0, 2.2, 1.0, 1.0, Choke),
    // Central spine you can vault onto to break sightlines down the corridor.
    (0.0, 1.3, 0.0, 3.0, 2.6, 3.0, Platform),
    (0.0, 0.5, 2.6, 2.4, 1.0, 1.6, Platform),
];
const L2_BEACONS: &[BeaconSpec] = &[
    (0.0, 14.0, [1.7, 0.5, 0.15]),
    (0.0, -14.0, [1.7, 0.5, 0.15]),
    (12.0, 0.0, [0.2, 1.4, 1.6]),
    (-12.0, 0.0, [0.2, 1.4, 1.6]),
];
const L2_SPAWNS: &[(f32, f32)] = &[
    (0.0, 17.0),
    (0.0, -17.0),
    (14.0, 11.0),
    (-14.0, 11.0),
    (14.0, -11.0),
    (-14.0, -11.0),
];
const L2_PADS: &[(f32, f32)] = &[(0.0, 4.0), (0.0, -4.0)];

const L3_BLOCKS: &[BlockSpec] = &[
    (9.0, 2.5, 0.0, 1.5, 5.0, 1.5, Pillar),
    (6.4, 2.5, 6.4, 1.5, 5.0, 1.5, Pillar),
    (0.0, 2.5, 9.0, 1.5, 5.0, 1.5, Pillar),
    (-6.4, 2.5, 6.4, 1.5, 5.0, 1.5, Pillar),
    (-9.0, 2.5, 0.0, 1.5, 5.0, 1.5, Pillar),
    (-6.4, 2.5, -6.4, 1.5, 5.0, 1.5, Pillar),
    (0.0, 2.5, -9.0, 1.5, 5.0, 1.5, Pillar),
    (6.4, 2.5, -6.4, 1.5, 5.0, 1.5, Pillar),
    (0.0, 0.5, 0.0, 2.4, 1.0, 2.4, Cover),
    // Raised gallery between two pillars: high ground over the colonnade.
    (13.0, 1.5, 0.0, 4.0, 3.0, 5.0, Platform),
    (10.0, 0.6, 0.0, 2.0, 1.2, 3.0, Platform),
];
const L3_BEACONS: &[BeaconSpec] = &[
    (0.0, 0.0, [0.3, 0.6, 1.8]),
    (13.0, 13.0, [1.5, 0.3, 1.4]),
    (-13.0, -13.0, [1.5, 0.3, 1.4]),
];
const L3_SPAWNS: &[(f32, f32)] = &[
    (15.0, 0.0),
    (-15.0, 0.0),
    (0.0, 15.0),
    (0.0, -15.0),
    (11.0, 11.0),
    (-11.0, -11.0),
];
const L3_PADS: &[(f32, f32)] = &[(15.5, 0.0), (-13.0, 13.0)];

const L4_BLOCKS: &[BlockSpec] = &[
    (11.0, 1.75, 0.0, 7.0, 3.5, 1.0, Wall),
    (-11.0, 1.75, 0.0, 7.0, 3.5, 1.0, Wall),
    (0.0, 1.75, 11.0, 1.0, 3.5, 7.0, Wall),
    (0.0, 1.75, -11.0, 1.0, 3.5, 7.0, Wall),
    (0.0, 2.0, 0.0, 2.2, 4.0, 2.2, Monument),
    (12.5, 0.45, 12.5, 1.4, 0.9, 1.4, Choke),
    (-12.5, 0.45, 12.5, 1.4, 0.9, 1.4, Choke),
    (12.5, 0.45, -12.5, 1.4, 0.9, 1.4, Choke),
    (-12.5, 0.45, -12.5, 1.4, 0.9, 1.4, Choke),
    // Twin elevated platforms flanking the monument.
    (8.0, 1.4, 0.0, 3.6, 2.8, 6.0, Platform),
    (-8.0, 1.4, 0.0, 3.6, 2.8, 6.0, Platform),
];
const L4_BEACONS: &[BeaconSpec] = &[
    (6.0, 6.0, [1.6, 0.3, 0.3]),
    (-6.0, 6.0, [1.6, 0.6, 0.2]),
    (6.0, -6.0, [1.6, 0.6, 0.2]),
    (-6.0, -6.0, [1.6, 0.3, 0.3]),
];
const L4_SPAWNS: &[(f32, f32)] = &[
    (16.0, 16.0),
    (-16.0, 16.0),
    (16.0, -16.0),
    (-16.0, -16.0),
    (16.0, 0.0),
    (-16.0, 0.0),
];
const L4_PADS: &[(f32, f32)] = &[(11.5, 0.0), (-11.5, 0.0)];

// ZIGGURAT — a stepped Doom-style pyramid you climb tier by tier to a perch.
const L5_BLOCKS: &[BlockSpec] = &[
    (0.0, 0.5, 0.0, 14.0, 1.0, 14.0, Platform),
    (0.0, 1.0, 0.0, 10.0, 2.0, 10.0, Platform),
    (0.0, 1.5, 0.0, 6.0, 3.0, 6.0, Platform),
    (0.0, 2.0, 0.0, 3.0, 4.0, 3.0, Monument),
    (15.0, 1.0, 15.0, 2.0, 2.0, 2.0, Pillar),
    (-15.0, 1.0, -15.0, 2.0, 2.0, 2.0, Pillar),
];
const L5_RAMPS: &[RampSpec] = &[
    (0.0, 0.5, 9.0, 4.0, 0.4, 4.0, 0.30, 0.0),
    (
        9.0,
        0.5,
        0.0,
        4.0,
        0.4,
        4.0,
        0.30,
        std::f32::consts::FRAC_PI_2,
    ),
];
const L5_BEACONS: &[BeaconSpec] = &[
    (0.0, 0.0, [1.7, 1.0, 0.3]),
    (13.0, 13.0, [0.3, 1.4, 1.6]),
    (-13.0, -13.0, [1.5, 0.3, 1.2]),
];
const L5_SPAWNS: &[(f32, f32)] = &[
    (16.0, 0.0),
    (-16.0, 0.0),
    (0.0, 16.0),
    (0.0, -16.0),
    (12.0, 12.0),
    (-12.0, -12.0),
];
const L5_PADS: &[(f32, f32)] = &[(8.0, 8.0), (-8.0, -8.0)];

// CHASM — a raised perimeter walkway around a sunken pit; ramps and pads link
// the two heights so the fight flows up and down.
const L6_BLOCKS: &[BlockSpec] = &[
    (0.0, 1.25, 13.5, 30.0, 2.5, 5.0, Platform),
    (0.0, 1.25, -13.5, 30.0, 2.5, 5.0, Platform),
    (13.5, 1.25, 0.0, 5.0, 2.5, 22.0, Platform),
    (-13.5, 1.25, 0.0, 5.0, 2.5, 22.0, Platform),
    (0.0, 0.5, 0.0, 6.0, 1.0, 6.0, Cover),
];
const L6_RAMPS: &[RampSpec] = &[
    (0.0, 0.9, 8.5, 5.0, 0.4, 5.0, 0.42, 0.0),
    (0.0, 0.9, -8.5, 5.0, 0.4, 5.0, -0.42, 0.0),
];
const L6_BEACONS: &[BeaconSpec] = &[
    (0.0, 0.0, [0.3, 0.7, 1.8]),
    (13.5, 13.5, [1.6, 0.4, 0.3]),
    (-13.5, -13.5, [1.6, 0.4, 0.3]),
];
const L6_SPAWNS: &[(f32, f32)] = &[
    (0.0, 16.0),
    (0.0, -16.0),
    (16.0, 0.0),
    (-16.0, 0.0),
    (0.0, 4.0),
    (0.0, -4.0),
];
const L6_PADS: &[(f32, f32)] = &[(6.0, 0.0), (-6.0, 0.0)];

// SPIRE — a tall central column ringed by staggered ledges; verticality is the
// whole point and the gargoyles own the air.
const L7_BLOCKS: &[BlockSpec] = &[
    (0.0, 4.0, 0.0, 3.0, 8.0, 3.0, Monument),
    (6.0, 0.75, 0.0, 4.0, 1.5, 4.0, Platform),
    (-6.0, 1.5, 0.0, 4.0, 3.0, 4.0, Platform),
    (0.0, 2.25, 6.0, 4.0, 4.5, 4.0, Platform),
    (0.0, 1.1, -6.0, 4.0, 2.2, 4.0, Platform),
    (12.0, 1.0, 12.0, 3.0, 2.0, 3.0, Pillar),
    (-12.0, 1.0, -12.0, 3.0, 2.0, 3.0, Pillar),
];
const L7_RAMPS: &[RampSpec] = &[(3.5, 0.6, 0.0, 3.0, 0.4, 4.0, 0.0, 0.0)];
const L7_BEACONS: &[BeaconSpec] = &[
    (0.0, 0.0, [0.7, 0.4, 1.8]),
    (10.0, -10.0, [0.2, 1.5, 1.4]),
    (-10.0, 10.0, [1.6, 0.7, 0.2]),
];
const L7_SPAWNS: &[(f32, f32)] = &[
    (15.0, 15.0),
    (-15.0, -15.0),
    (15.0, -15.0),
    (-15.0, 15.0),
    (0.0, 16.0),
    (16.0, 0.0),
];
const L7_PADS: &[(f32, f32)] = &[(6.0, 0.0), (-6.0, 0.0), (0.0, 6.0)];

const LEVELS: &[Level] = &[
    Level {
        name: "ARRIVAL",
        atmosphere: Atmosphere::Nebula,
        fog: [0.05, 0.02, 0.10],
        spawn: [0.0, 1.2, 14.0],
        exit: [0.0, -16.5],
        blocks: L1_BLOCKS,
        ramps: &[],
        beacons: L1_BEACONS,
        spawn_points: L1_SPAWNS,
        pads: L1_PADS,
        roster: Roster {
            imps: 7,
            swarmers: 4,
            casters: 1,
            brutes: 0,
            gargoyles: 0,
            sentinels: 0,
        },
    },
    Level {
        name: "GAUNTLET",
        atmosphere: Atmosphere::Sunset,
        fog: [0.12, 0.04, 0.02],
        spawn: [0.0, 1.2, 16.0],
        exit: [0.0, -16.5],
        blocks: L2_BLOCKS,
        ramps: &[],
        beacons: L2_BEACONS,
        spawn_points: L2_SPAWNS,
        pads: L2_PADS,
        roster: Roster {
            imps: 6,
            swarmers: 7,
            casters: 2,
            brutes: 1,
            gargoyles: 1,
            sentinels: 1,
        },
    },
    Level {
        name: "COLONNADE",
        atmosphere: Atmosphere::Space,
        fog: [0.02, 0.03, 0.08],
        spawn: [0.0, 1.2, 15.5],
        exit: [0.0, -16.5],
        blocks: L3_BLOCKS,
        ramps: &[],
        beacons: L3_BEACONS,
        spawn_points: L3_SPAWNS,
        pads: L3_PADS,
        roster: Roster {
            imps: 4,
            swarmers: 5,
            casters: 5,
            brutes: 1,
            gargoyles: 2,
            sentinels: 2,
        },
    },
    Level {
        name: "CRUCIBLE",
        atmosphere: Atmosphere::Nebula,
        fog: [0.10, 0.03, 0.04],
        spawn: [0.0, 1.2, 16.5],
        exit: [0.0, -16.5],
        blocks: L4_BLOCKS,
        ramps: &[],
        beacons: L4_BEACONS,
        spawn_points: L4_SPAWNS,
        pads: L4_PADS,
        roster: Roster {
            imps: 9,
            swarmers: 8,
            casters: 3,
            brutes: 2,
            gargoyles: 2,
            sentinels: 2,
        },
    },
    Level {
        name: "ZIGGURAT",
        atmosphere: Atmosphere::Sunset,
        fog: [0.10, 0.05, 0.02],
        spawn: [0.0, 1.2, 16.0],
        exit: [0.0, -16.5],
        blocks: L5_BLOCKS,
        ramps: L5_RAMPS,
        beacons: L5_BEACONS,
        spawn_points: L5_SPAWNS,
        pads: L5_PADS,
        roster: Roster {
            imps: 8,
            swarmers: 6,
            casters: 3,
            brutes: 1,
            gargoyles: 2,
            sentinels: 1,
        },
    },
    Level {
        name: "CHASM",
        atmosphere: Atmosphere::Space,
        fog: [0.02, 0.04, 0.10],
        spawn: [0.0, 3.0, 16.0],
        exit: [0.0, -16.5],
        blocks: L6_BLOCKS,
        ramps: L6_RAMPS,
        beacons: L6_BEACONS,
        spawn_points: L6_SPAWNS,
        pads: L6_PADS,
        roster: Roster {
            imps: 6,
            swarmers: 6,
            casters: 4,
            brutes: 1,
            gargoyles: 3,
            sentinels: 2,
        },
    },
    Level {
        name: "SPIRE",
        atmosphere: Atmosphere::Nebula,
        fog: [0.06, 0.03, 0.10],
        spawn: [0.0, 1.2, 16.0],
        exit: [0.0, -16.5],
        blocks: L7_BLOCKS,
        ramps: L7_RAMPS,
        beacons: L7_BEACONS,
        spawn_points: L7_SPAWNS,
        pads: L7_PADS,
        roster: Roster {
            imps: 5,
            swarmers: 5,
            casters: 3,
            brutes: 1,
            gargoyles: 5,
            sentinels: 2,
        },
    },
];

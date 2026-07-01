//! Hand-authored level definitions. Each level is a distinct *place* — its own
//! footprint, room/corridor architecture, height structure, spawn, and exit
//! gate — not a reskin of one arena. Walls (full height) partition space into
//! rooms and corridors with doorway gaps; platforms make raised galleries and
//! catwalks reached by pads; the exit sits at the end of a navigable path.

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
    /// Emissive structural block — reactor cores, altars, glowing machinery.
    Core,
}

impl BlockKind {
    /// The editor's placeable palette, in display order (Core is hand-authored only).
    pub const ALL: [BlockKind; 6] = [
        BlockKind::Wall,
        BlockKind::Pillar,
        BlockKind::Platform,
        BlockKind::Cover,
        BlockKind::Choke,
        BlockKind::Monument,
    ];

    /// Wire order for [`BlockKind::code`] / [`BlockKind::from_code`]. The single
    /// source of truth for on-disk block codes: both directions derive from it, so
    /// a new variant can never desync the two halves of the mapping. Append only —
    /// reordering rewrites the meaning of every saved level file.
    const ORDER: [BlockKind; 7] = [
        BlockKind::Wall,
        BlockKind::Pillar,
        BlockKind::Cover,
        BlockKind::Choke,
        BlockKind::Monument,
        BlockKind::Platform,
        BlockKind::Core,
    ];

    pub fn label(self) -> &'static str {
        match self {
            BlockKind::Wall => "WALL",
            BlockKind::Pillar => "PILLAR",
            BlockKind::Cover => "COVER",
            BlockKind::Choke => "CHOKE",
            BlockKind::Monument => "MONUMENT",
            BlockKind::Platform => "PLATFORM",
            BlockKind::Core => "CORE",
        }
    }

    pub fn code(self) -> u8 {
        Self::ORDER
            .iter()
            .position(|&kind| kind == self)
            .map(|index| index as u8)
            .unwrap_or(0)
    }

    pub fn from_code(code: u8) -> BlockKind {
        Self::ORDER
            .get(code as usize)
            .copied()
            .unwrap_or(BlockKind::Wall)
    }
}

/// (cx, cy, cz, sx, sy, sz, kind)
pub type BlockSpec = (f32, f32, f32, f32, f32, f32, BlockKind);
/// (x, z, [r, g, b])
pub type BeaconSpec = (f32, f32, [f32; 3]);
/// (cx, cy, cz, sx, sy, sz, pitch_radians, yaw_radians) — a tilted slab you can
/// walk up. Keep pitch under ~27 degrees so the controller climbs without sliding.
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

pub struct Level {
    pub name: &'static str,
    pub atmosphere: Atmosphere,
    pub fog: [f32; 3],
    /// Footprint half-extents; the floor and perimeter walls are sized to these,
    /// so levels are different shapes and sizes, not one square box.
    pub half_x: f32,
    pub half_z: f32,
    pub spawn: [f32; 3],
    pub exit: [f32; 2],
    pub blocks: &'static [BlockSpec],
    pub ramps: &'static [RampSpec],
    pub beacons: &'static [BeaconSpec],
    pub spawn_points: &'static [(f32, f32)],
    pub pads: &'static [(f32, f32)],
    pub roster: Roster,
}

/// An owned, mutable level — what the in-game editor builds and what custom
/// play sessions run from.
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

pub fn count() -> usize {
    LEVELS.len()
}

pub fn level(index: usize) -> &'static Level {
    &LEVELS[index % LEVELS.len()]
}

use BlockKind::{Core, Cover, Monument, Pillar, Platform, Wall};

// L0 — THE FOUNDRY: reactor hall, walkable apron, two gantry towers via pads.
const L0_BLOCKS: &[BlockSpec] = &[
    (0.0, 3.0, 0.0, 14.0, 6.0, 12.0, Monument), // reactor housing, top 6
    (0.0, 8.5, 0.0, 6.0, 5.0, 6.0, Core),       // glowing reactor core, top 11
    (0.0, 0.75, 0.0, 22.0, 1.5, 18.0, Platform), // walkable reactor apron, top 1.5
    (13.0, 3.5, 9.0, 2.2, 7.0, 2.2, Pillar),    // coolant towers
    (-13.0, 3.5, 9.0, 2.2, 7.0, 2.2, Pillar),
    (13.0, 3.5, -9.0, 2.2, 7.0, 2.2, Pillar),
    (-13.0, 3.5, -9.0, 2.2, 7.0, 2.2, Pillar),
    (41.0, 3.0, 0.0, 6.0, 6.0, 30.0, Platform), // east gantry deck, top 6
    (30.0, 1.5, 12.0, 8.0, 3.0, 6.0, Platform), // east catwalk step, top 3
    (30.0, 1.5, -12.0, 8.0, 3.0, 6.0, Platform),
    (-41.0, 3.0, 0.0, 6.0, 6.0, 30.0, Platform), // west gantry deck, top 6
    (-30.0, 1.5, 12.0, 8.0, 3.0, 6.0, Platform), // west catwalk step, top 3
    (-30.0, 1.5, -12.0, 8.0, 3.0, 6.0, Platform),
    (13.0, 1.0, 28.0, 5.0, 2.0, 3.0, Cover), // entry cover
    (-13.0, 1.0, 28.0, 5.0, 2.0, 3.0, Cover),
    (13.0, 1.0, -30.0, 5.0, 2.0, 3.0, Cover), // gate cover
    (-13.0, 1.0, -30.0, 5.0, 2.0, 3.0, Cover),
];
const L0_BEACONS: &[BeaconSpec] = &[
    (0.0, 9.0, [2.3, 0.9, 0.25]), // reactor uplights, hot orange
    (0.0, -9.0, [2.3, 0.9, 0.25]),
    (14.0, 0.0, [1.9, 0.55, 0.15]),
    (-14.0, 0.0, [1.9, 0.55, 0.15]),
    (38.0, 14.0, [0.2, 1.2, 1.8]), // gantry markers, cyan
    (-38.0, -14.0, [0.2, 1.2, 1.8]),
    (0.0, 32.0, [0.35, 0.5, 0.85]), // entry, cold
    (0.0, -33.0, [1.9, 0.5, 0.3]),  // gate, warm
];
const L0_SPAWNS: &[(f32, f32)] = &[
    (22.0, 24.0),
    (-22.0, 24.0),
    (30.0, 0.0),
    (-30.0, 0.0),
    (22.0, -24.0),
    (-22.0, -24.0),
    (0.0, 22.0),
    (0.0, -22.0),
];
const L0_PADS: &[(f32, f32)] = &[(30.0, 20.0), (-30.0, -20.0), (30.0, -20.0), (-30.0, 20.0)];

// L1 — THE SPIRE: central climbable spire, four raised corner balconies via pads.
const L1_BLOCKS: &[BlockSpec] = &[
    (0.0, 4.0, 0.0, 8.0, 8.0, 8.0, Monument), // spire base, top 8
    (0.0, 11.0, 0.0, 5.0, 6.0, 5.0, Monument), // spire shaft, top 14
    (0.0, 16.0, 0.0, 3.0, 4.0, 3.0, Core),    // glowing crown, top 18
    (9.0, 1.5, 0.0, 5.0, 3.0, 5.0, Platform), // ascent ledge, top 3
    (0.0, 2.5, 9.0, 5.0, 5.0, 5.0, Platform), // ascent ledge, top 5
    (-9.0, 3.5, 0.0, 5.0, 7.0, 5.0, Platform), // ascent ledge, top 7
    (0.0, 2.5, -9.0, 5.0, 5.0, 5.0, Platform), // ascent ledge, top 5
    (30.0, 2.0, 30.0, 12.0, 4.0, 12.0, Platform), // NE balcony, top 4
    (-30.0, 2.0, 30.0, 12.0, 4.0, 12.0, Platform), // NW balcony
    (30.0, 2.0, -30.0, 12.0, 4.0, 12.0, Platform), // SE balcony
    (-30.0, 2.0, -30.0, 12.0, 4.0, 12.0, Platform), // SW balcony
    (14.0, 0.6, -14.0, 3.0, 1.2, 3.0, Cover), // pit cover
    (-14.0, 0.6, 14.0, 3.0, 1.2, 3.0, Cover),
];
const L1_BEACONS: &[BeaconSpec] = &[
    (0.0, 0.0, [0.7, 0.35, 1.9]), // spire crown, violet
    (30.0, 30.0, [0.3, 1.4, 1.3]),
    (-30.0, -30.0, [1.7, 0.7, 0.2]),
    (30.0, -30.0, [0.3, 1.4, 1.3]),
    (-30.0, 30.0, [1.7, 0.7, 0.2]),
    (0.0, 34.0, [0.35, 0.5, 0.85]),
    (0.0, -35.0, [1.6, 0.4, 0.4]),
];
const L1_SPAWNS: &[(f32, f32)] = &[
    (0.0, 24.0),
    (24.0, 0.0),
    (-24.0, 0.0),
    (0.0, -24.0),
    (16.0, 16.0),
    (-16.0, -16.0),
    (30.0, 30.0),
    (-30.0, -30.0),
];
const L1_PADS: &[(f32, f32)] = &[
    (9.0, 9.0),
    (-9.0, -9.0),
    (24.0, 24.0),
    (-24.0, 24.0),
    (24.0, -24.0),
    (-24.0, -24.0),
];

// L2 — THE WARRENS: three chambers, doorways alternating sides, NW keycard vault.
const L2_BLOCKS: &[BlockSpec] = &[
    (-13.0, 5.0, 8.0, 62.0, 10.0, 1.5, Wall), // divider 1 (x[-44,18]); doorway east
    (37.0, 5.0, 8.0, 14.0, 10.0, 1.5, Wall),  // divider 1 east span (x[30,44])
    (13.0, 5.0, -8.0, 62.0, 10.0, 1.5, Wall), // divider 2 (x[-18,44]); doorway west
    (-37.0, 5.0, -8.0, 14.0, 10.0, 1.5, Wall), // divider 2 west span (x[-44,-30])
    (-28.0, 5.0, -30.0, 1.5, 10.0, 8.0, Wall), // vault east wall (z[-34,-26])
    (-28.0, 5.0, -19.0, 1.5, 10.0, 6.0, Wall), // vault east wall (z[-22,-16]); doorway z[-26,-22]
    (-36.0, 5.0, -16.5, 17.0, 10.0, 1.5, Wall), // vault south wall (x[-44.5,-27.5])
    (-36.0, 0.6, -26.0, 2.0, 1.2, 2.0, Core), // keycard shrine pedestal
    (-39.0, 0.6, -31.0, 3.0, 1.2, 1.4, Cover), // cover inside the vault
    (0.0, 0.9, 20.0, 4.0, 1.8, 1.4, Cover),   // entry chamber cover
    (10.0, 0.9, 0.0, 4.0, 1.8, 1.4, Cover),   // mid chamber cover
    (-10.0, 0.9, 0.0, 4.0, 1.8, 1.4, Cover),
    (8.0, 2.5, -22.0, 2.0, 5.0, 2.0, Pillar), // north chamber pillars
    (-8.0, 2.5, -22.0, 2.0, 5.0, 2.0, Pillar),
];
const L2_BEACONS: &[BeaconSpec] = &[
    (-36.0, -26.0, [2.0, 1.4, 0.3]), // vault shrine, hot gold
    (24.0, 8.0, [0.3, 1.3, 1.6]),    // east doorway, cyan
    (-24.0, -8.0, [1.7, 0.4, 0.3]),  // west doorway, red
    (0.0, 20.0, [0.4, 0.5, 0.9]),
    (0.0, 0.0, [0.4, 0.5, 0.9]),
    (0.0, -28.0, [1.6, 0.45, 0.3]), // gate
];
const L2_SPAWNS: &[(f32, f32)] = &[
    (20.0, 22.0),
    (-20.0, 22.0),
    (20.0, 0.0),
    (-20.0, 0.0),
    (16.0, -22.0),
    (-16.0, -24.0),
    (0.0, -12.0),
    (0.0, 14.0),
];
const L2_PADS: &[(f32, f32)] = &[];

// L3 — THE CRUCIBLE: colonnade nave to a raised throne, flanking side aisles via pads.
const L3_BLOCKS: &[BlockSpec] = &[
    (0.0, 1.5, -34.0, 18.0, 3.0, 8.0, Platform), // throne dais, top 3
    (0.0, 4.5, -37.0, 6.0, 6.0, 3.0, Monument),  // throne back
    (-12.0, 5.0, -35.0, 4.0, 10.0, 4.0, Monument), // left monolith
    (12.0, 5.0, -35.0, 4.0, 10.0, 4.0, Monument), // right monolith
    (0.0, 7.0, -37.0, 2.5, 3.0, 2.5, Core),      // throne crown, glow
    (-9.0, 3.5, 20.0, 2.5, 7.0, 2.5, Pillar),    // nave colonnade
    (9.0, 3.5, 20.0, 2.5, 7.0, 2.5, Pillar),
    (-9.0, 3.5, 8.0, 2.5, 7.0, 2.5, Pillar),
    (9.0, 3.5, 8.0, 2.5, 7.0, 2.5, Pillar),
    (-9.0, 3.5, -4.0, 2.5, 7.0, 2.5, Pillar),
    (9.0, 3.5, -4.0, 2.5, 7.0, 2.5, Pillar),
    (-9.0, 3.5, -16.0, 2.5, 7.0, 2.5, Pillar),
    (9.0, 3.5, -16.0, 2.5, 7.0, 2.5, Pillar),
    (28.0, 2.0, 0.0, 8.0, 4.0, 40.0, Platform), // east aisle, top 4
    (-28.0, 2.0, 0.0, 8.0, 4.0, 40.0, Platform), // west aisle, top 4
    (0.0, 0.8, 30.0, 6.0, 1.6, 2.0, Cover),     // entrance cover
    (0.0, 0.8, 0.0, 4.0, 1.6, 4.0, Cover),      // central altar cover
];
const L3_BEACONS: &[BeaconSpec] = &[
    (0.0, -36.0, [2.3, 0.5, 0.18]), // throne, hot red
    (-12.0, -35.0, [1.9, 0.55, 0.15]),
    (12.0, -35.0, [1.9, 0.55, 0.15]),
    (28.0, 10.0, [0.35, 0.4, 0.8]), // aisles, cold
    (-28.0, -10.0, [0.35, 0.4, 0.8]),
    (0.0, 34.0, [0.4, 0.45, 0.8]), // entrance
];
const L3_SPAWNS: &[(f32, f32)] = &[
    (0.0, -34.0), // throne (boss)
    (20.0, -20.0),
    (-20.0, -20.0),
    (28.0, 0.0),
    (-28.0, 0.0),
    (12.0, 16.0),
    (-12.0, 16.0),
    (0.0, 28.0),
];
const L3_PADS: &[(f32, f32)] = &[(20.0, 12.0), (-20.0, 12.0), (20.0, -12.0), (-20.0, -12.0)];

const LEVELS: &[Level] = &[
    Level {
        name: "THE FOUNDRY",
        atmosphere: Atmosphere::Sunset,
        fog: [0.10, 0.05, 0.02],
        half_x: 46.0,
        half_z: 38.0,
        spawn: [0.0, 1.2, 32.0],
        exit: [0.0, -34.0],
        blocks: L0_BLOCKS,
        ramps: &[],
        beacons: L0_BEACONS,
        spawn_points: L0_SPAWNS,
        pads: L0_PADS,
        roster: Roster {
            imps: 10,
            swarmers: 8,
            casters: 3,
            brutes: 1,
            gargoyles: 1,
            sentinels: 2,
        },
    },
    Level {
        name: "THE SPIRE",
        atmosphere: Atmosphere::Nebula,
        fog: [0.06, 0.03, 0.10],
        half_x: 40.0,
        half_z: 40.0,
        spawn: [0.0, 1.2, 34.0],
        exit: [0.0, -36.0],
        blocks: L1_BLOCKS,
        ramps: &[],
        beacons: L1_BEACONS,
        spawn_points: L1_SPAWNS,
        pads: L1_PADS,
        roster: Roster {
            imps: 6,
            swarmers: 6,
            casters: 4,
            brutes: 1,
            gargoyles: 6,
            sentinels: 3,
        },
    },
    Level {
        name: "THE WARRENS",
        atmosphere: Atmosphere::Space,
        fog: [0.02, 0.03, 0.08],
        half_x: 44.0,
        half_z: 34.0,
        spawn: [0.0, 1.2, 28.0],
        exit: [0.0, -30.0],
        blocks: L2_BLOCKS,
        ramps: &[],
        beacons: L2_BEACONS,
        spawn_points: L2_SPAWNS,
        pads: L2_PADS,
        roster: Roster {
            imps: 9,
            swarmers: 8,
            casters: 4,
            brutes: 2,
            gargoyles: 1,
            sentinels: 3,
        },
    },
    Level {
        name: "THE CRUCIBLE",
        atmosphere: Atmosphere::Nebula,
        fog: [0.10, 0.03, 0.04],
        half_x: 42.0,
        half_z: 44.0,
        spawn: [0.0, 1.2, 38.0],
        exit: [0.0, -40.0],
        blocks: L3_BLOCKS,
        ramps: &[],
        beacons: L3_BEACONS,
        spawn_points: L3_SPAWNS,
        pads: L3_PADS,
        roster: Roster {
            imps: 12,
            swarmers: 10,
            casters: 4,
            brutes: 3,
            gargoyles: 3,
            sentinels: 3,
        },
    },
];

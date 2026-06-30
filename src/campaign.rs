//! The Story-mode campaign: an ordered run of themed missions over the hand-
//! built arenas, each with an objective and narrative framing, stitched
//! together by text cutscenes.

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Objective {
    /// Wipe out every wave, then the gate opens.
    #[default]
    Exterminate,
    /// The gate is open from the start — punch through the horde and escape.
    Reach,
    /// A warlord anchors the final wave; clear the floor to open the gate.
    Boss,
    /// The gate is locked until you find and grab the keycard across the level.
    Keycard,
}

impl Objective {
    pub fn label(self) -> &'static str {
        match self {
            Objective::Exterminate => "EXTERMINATE THE HORDE",
            Objective::Reach => "REACH THE GATE",
            Objective::Boss => "KILL THE WARLORD",
            Objective::Keycard => "RECOVER THE KEYCARD",
        }
    }
}

pub struct Mission {
    /// Index into the static `LEVELS` table whose geometry/theme this mission uses.
    pub level: usize,
    pub title: &'static str,
    pub objective: Objective,
    /// World position of the keycard for `Keycard` missions; ignored otherwise.
    pub key: [f32; 3],
    pub briefing: &'static str,
    pub debrief: &'static str,
}

pub fn count() -> usize {
    CAMPAIGN.len()
}

pub fn mission(index: usize) -> &'static Mission {
    &CAMPAIGN[index.min(CAMPAIGN.len() - 1)]
}

/// Opening cutscene, shown once before the first mission.
pub const INTRO: &[&str] = &[
    "The colony ship GEHENNA went dark over the ring-world eight days ago.\n\nYou are the only marine still breathing.",
    "Something down there turned the crew into the horde now boiling across the decks.\n\nCut a path to the core. Put it down. Get out.",
];

/// Closing cutscene, shown after the final mission.
pub const ENDING: &[&str] = &[
    "The overlord folds in on itself and the ring-world goes quiet.\n\nFor the first time in eight days, nothing is trying to kill you.",
    "GEHENNA drifts, dead and silent, and you are still breathing.\n\nRIP AND TEAR ACCOMPLISHED.",
];

pub const CAMPAIGN: &[Mission] = &[
    Mission {
        level: 0,
        title: "THE FOUNDRY",
        objective: Objective::Exterminate,
        key: [0.0, 0.0, 0.0],
        briefing: "You hit the foundry deck hard. A reactor block fills the hall — circle it and clear every last one of them. Watch the roof; it's the high ground.",
        debrief: "Foundry cold. Ahead, a flooded lock corridor runs deeper into the ship.",
    },
    Mission {
        level: 1,
        title: "THE LOCKS",
        objective: Objective::Reach,
        key: [0.0, 0.0, 0.0],
        briefing: "The lock corridor is a kill-channel of bulkheads and the doors won't hold. Don't dig in — weave through the gates and reach the far end before it closes on you.",
        debrief: "Last lock sealed behind you. Whatever's herding them is close now.",
    },
    Mission {
        level: 2,
        title: "THE GALLERY",
        objective: Objective::Exterminate,
        key: [0.0, 0.0, 0.0],
        briefing: "Casters and sentinels hold the gallery balcony while the rushers boil in the pit. Use the pads, take the high ring, and burn the whole room down.",
        debrief: "The gallery is ash. The deck climbs into an open spire hall.",
    },
    Mission {
        level: 3,
        title: "SPIRE HALL",
        objective: Objective::Boss,
        key: [0.0, 0.0, 0.0],
        briefing: "A warlord holds the spire and the air around it is thick with wings. Ride the pads up the ledges, break the flyers, and put the warlord down.",
        debrief: "The warlord is meat. Past the spire, the warrens twist toward the core.",
    },
    Mission {
        level: 4,
        title: "THE WARRENS",
        objective: Objective::Keycard,
        key: [-12.0, 0.0, 8.0],
        briefing: "The core gate is locked. The keycard is sealed in the west chamber off the spine — get in, take it, then run the spine to the gate.",
        debrief: "Keycard in hand, the warrens fall behind you. The core chamber glows red ahead.",
    },
    Mission {
        level: 5,
        title: "THE CRUCIBLE",
        objective: Objective::Boss,
        key: [0.0, 0.0, 0.0],
        briefing: "This is the core. The overlord is here and everything it has left stands between you and it. No exit until it's dead.",
        debrief: "Silence.",
    },
];

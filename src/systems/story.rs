//! Story-mode director: sequences cutscenes and missions, threading the
//! campaign from the opening transmission through each briefing, mission, and
//! debrief to the ending.

use crate::campaign;
use crate::ecs::{BoomerWorld, Screen, StoryNext, StorySlide};
use crate::systems::lifecycle;
use crate::systems::world::game;
use nightshade::prelude::*;

pub fn begin(boomer_world: &mut BoomerWorld, world: &mut World) {
    boomer_world.resources.story.active = true;
    boomer_world.resources.story.mission = 0;
    let mut slides = intro_slides();
    slides.push(briefing_slide(0));
    show(boomer_world, world, slides, StoryNext::StartMission(0));
}

pub fn mission_complete(boomer_world: &mut BoomerWorld, world: &mut World) {
    let index = boomer_world.resources.story.mission;
    let mut slides = vec![debrief_slide(index)];
    let after = if index + 1 < campaign::count() {
        slides.push(briefing_slide(index + 1));
        StoryNext::StartMission(index + 1)
    } else {
        slides.extend(ending_slides());
        StoryNext::Title
    };
    show(boomer_world, world, slides, after);
}

/// Advance the on-screen cutscene; when the slides run out, do the queued action.
pub fn advance(boomer_world: &mut BoomerWorld, world: &mut World) {
    let count = boomer_world.resources.story.slides.len();
    let next_index = boomer_world.resources.story.slide_index + 1;
    if next_index < count {
        boomer_world.resources.story.slide_index = next_index;
        return;
    }
    match boomer_world.resources.story.after {
        StoryNext::StartMission(index) => start_mission(boomer_world, world, index),
        StoryNext::Title => {
            boomer_world.resources.story.active = false;
            game::start_at(boomer_world, world, 0);
            lifecycle::enter(boomer_world, world, Screen::Title);
        }
    }
}

fn start_mission(boomer_world: &mut BoomerWorld, world: &mut World, index: usize) {
    boomer_world.resources.story.mission = index;
    game::start_mission(boomer_world, world, index);
    lifecycle::enter(boomer_world, world, Screen::InGame);
}

fn show(
    boomer_world: &mut BoomerWorld,
    world: &mut World,
    slides: Vec<StorySlide>,
    after: StoryNext,
) {
    boomer_world.resources.story.slides = slides;
    boomer_world.resources.story.slide_index = 0;
    boomer_world.resources.story.after = after;
    lifecycle::enter(boomer_world, world, Screen::Cutscene);
}

fn slide(title: impl Into<String>, body: impl Into<String>) -> StorySlide {
    StorySlide {
        title: title.into(),
        body: body.into(),
    }
}

fn intro_slides() -> Vec<StorySlide> {
    campaign::INTRO
        .iter()
        .map(|body| slide("INCOMING TRANSMISSION", *body))
        .collect()
}

fn ending_slides() -> Vec<StorySlide> {
    campaign::ENDING
        .iter()
        .map(|body| slide("GEHENNA", *body))
        .collect()
}

fn briefing_slide(index: usize) -> StorySlide {
    let mission = campaign::mission(index);
    slide(
        format!("MISSION {}: {}", index + 1, mission.title),
        format!(
            "OBJECTIVE — {}\n\n{}",
            mission.objective.label(),
            mission.briefing
        ),
    )
}

fn debrief_slide(index: usize) -> StorySlide {
    let mission = campaign::mission(index);
    slide("MISSION COMPLETE", mission.debrief)
}

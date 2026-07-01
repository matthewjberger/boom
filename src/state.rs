use crate::ecs::{CobaltWorld, Phase, Screen};
use crate::systems;
use nightshade::ecs::camera::systems::first_person_camera_look_system;
use nightshade::prelude::*;

#[derive(Default)]
pub struct Brimstone {
    pub cobalt_world: CobaltWorld,
}

impl State for Brimstone {
    fn initialize(&mut self, world: &mut World) {
        world.resources.window.title = "BRIMSTONE".to_string();
        systems::lifecycle::initialize(&mut self.cobalt_world, world);
    }

    fn run_systems(&mut self, world: &mut World) {
        self.run_game(world);
    }
}

impl Brimstone {
    fn run_game(&mut self, world: &mut World) {
        systems::input::handle_global(&mut self.cobalt_world, world);
        systems::screens::title::handle_input(&mut self.cobalt_world, world);
        systems::screens::level_select::handle_input(&mut self.cobalt_world, world);
        systems::screens::mission_select::handle_input(&mut self.cobalt_world, world);
        systems::screens::pause::handle_input(&mut self.cobalt_world, world);
        systems::screens::cutscene::handle_input(&mut self.cobalt_world, world);

        if matches!(self.cobalt_world.resources.screen.current, Screen::Editor) {
            systems::editor::update(&mut self.cobalt_world, world);
            systems::world::fx::tick(&mut self.cobalt_world, world);
        }

        if matches!(self.cobalt_world.resources.screen.current, Screen::InGame) {
            let delta = world.resources.window.timing.delta_time.clamp(0.0, 0.1);
            let playing = matches!(self.cobalt_world.resources.game.phase, Phase::Playing);
            let frozen = {
                let game = &mut self.cobalt_world.resources.game;
                if game.hitstop > 0.0 {
                    game.hitstop -= delta;
                    true
                } else {
                    false
                }
            };

            let sim_active = playing && !frozen;
            self.cobalt_world.resources.player.sim_active = sim_active;
            if sim_active {
                systems::world::player::pre_look(&self.cobalt_world, world);
                first_person_camera_look_system(world);
                systems::world::player::movement(&mut self.cobalt_world, world);
                systems::world::weapon::update(&mut self.cobalt_world, world);
                systems::world::enemies::update(&mut self.cobalt_world, world);
                systems::world::projectiles::update(&mut self.cobalt_world, world);
                systems::world::pickups::update(&mut self.cobalt_world, world);
                systems::world::game::tick(&mut self.cobalt_world, world);
            }

            systems::world::player::apply_camera_feel(&mut self.cobalt_world, world);
            systems::world::billboard::update(&mut self.cobalt_world, world);
            systems::world::fx::tick(&mut self.cobalt_world, world);
            update_vfx_system(world);
            systems::world::viewmodel::update(&mut self.cobalt_world, world);
        }

        crate::adventure::update(&mut self.cobalt_world, world);

        systems::world::audio::tick(&mut self.cobalt_world, world);
        systems::screens::hud::update(&self.cobalt_world, world);
        systems::screens::cutscene::update(&mut self.cobalt_world, world);
    }
}

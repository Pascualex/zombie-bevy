use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    server::{resources::*, systems::*},
    TIME_STEP,
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::Lobby)
            .init_resource::<PlayersInfo>()
            .add_system_set_to_stage(
                CoreStage::Update,
                SystemSet::new()
                    .with_system(game_start)
                    .with_system(controller.after(game_start)),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(tick.after(controller)),
            );
    }
}

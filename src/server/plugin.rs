use bevy::{prelude::*, time::FixedTimestep};

use crate::server::{setup, systems::*};

use super::resources::EntitiesIds;

pub const TIME_STEP: f64 = 1.0 / 60.0;

#[derive(Default)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntitiesIds::new())
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP))
                    .with_system(join)
                    .with_system(movement.after(join))
                    .with_system(movement_input.after(movement))
                    .with_system(movement_sync.after(movement_input)),
            );
    }
}

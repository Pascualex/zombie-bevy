use bevy::prelude::*;

use crate::client::game::{resources::*, systems::*};

pub struct ClientGamePlugin;

impl Plugin for ClientGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::Waiting)
            .init_resource::<PlayerEntities>()
            .init_resource::<SpacePartitioner>()
            .init_resource::<Ticks>()
            .insert_resource(Random::new(0))
            .insert_resource(Spawner::from_hertz(100.0))
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_system(tick)
                    .with_system(initialization.after(tick)),
            )
            .add_system_set_to_stage(
                CoreStage::Update,
                SystemSet::new()
                    .with_system(movement)
                    .with_system(enemies_movement.after(movement))
                    .with_system(players_movement.after(movement))
                    .with_system(space_partitioner.after(players_movement))
                    .with_system(players_attack.after(space_partitioner))
                    .with_system(spawn.after(movement)),
            )
            .add_system_to_stage(CoreStage::PostUpdate, death);
    }
}

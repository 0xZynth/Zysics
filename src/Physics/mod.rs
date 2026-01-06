use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;
pub use systems::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (apply_gravity, apply_velocity, collision_system).chain());
    }
}

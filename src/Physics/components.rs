use bevy::prelude::*;
use bevy::math::DVec2;



#[derive(Component, Default)]
pub struct RigidBody;

#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity(pub DVec2);

#[derive(Component)]
pub struct Mass(pub f64);

#[derive(Component)]
pub enum Collider {
    Circle(f64),
}

#[derive(Component)]
pub struct Restitution(pub f64);
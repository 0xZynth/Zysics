mod Physics;
use std::f64;

use Physics::*;
use bevy::prelude::*;
use bevy::math::DVec2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    
    // floor (just a big circle for now until other shapes are implmented or find a better way)
    commands.spawn((
        RigidBody,
        Velocity(DVec2::ZERO),
        Mass(0.0), // 0.0 mass = static/infinite for our simple logic (need to handle in system)
        Restitution(0.7),
        Collider::Circle(500000.0),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(1000000.0, 1000000.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -500000.0, 0.0),
    ));
}

fn spawn_input(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Ok((camera, camera_transform)) = camera.single() {
            if let Ok(window) = windows.single() {
                if let Some(cursor_position) = window.cursor_position() {
                    if let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                         commands.spawn((
                    RigidBody,
                    Velocity(DVec2::new(0.0, 0.0)),
                    Mass(1.0),
                    Restitution(0.7),
                    Collider::Circle(20.0),
                    Sprite {
                        color: Color::hsv(rand::random::<f32>() * 360.0, 1.0, 1.0),
                        custom_size: Some(Vec2::new(40.0, 40.0)),
                        ..default()
                    },
                    Transform::from_xyz(point.x, point.y, 0.0),
                ));
                    }
                }
            }
        }
    }
}

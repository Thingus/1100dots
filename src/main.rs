use bevy::{
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    post_process::bloom::{Bloom, BloomCompositeMode},
    prelude::*,
};
use rand::Rng;
use std::f32::consts::PI;
const ELECTRON_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const ELECTRON_SIZE: f32 = 3.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_electrons, move_electrons))
        .run();
}

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Electron {
    speed: Speed,
}

#[derive(Component)]
struct ElectronInfluencer {
    radius: f32,
    magnitude: f32,
}

#[derive(Component)]
struct ElectronEmitter;

#[derive(Component)]
struct ElectronCollector;

#[derive(Component)]
struct Collidable;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface, // 1. Using a tonemapper that desaturates to white is recommended
        Bloom {
            intensity: 0.75,
            ..default()
        }, // 2. Enable bloom for the camera
        DebandDither::Enabled,      // Optional: bloom causes gradients which cause banding
    ));

    commands.spawn((
        ElectronEmitter,
        Transform {
            translation: Vec3::new(-300., -30., 1.),
            ..default()
        },
    ));
}

fn spawn_electrons(
    mut commands: Commands,
    collider_query: Query<(&ElectronEmitter, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    for (emitter, emitter_transform) in collider_query {
        let angle = rng.random_range(-10. * PI..10. * PI);
        let speed = rng.random_range(5.0..10.0);
        commands.spawn((
            Electron {
                speed: Speed(speed),
            },
            emitter_transform
                .with_scale(Vec3::new(1., 1., 1.))
                .with_rotation(Quat::from_rotation_z(angle)),
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(ELECTRON_COLOR)),
        ));
    }
}

fn move_electrons(query: Query<(&Electron, &mut Transform)>, time: Res<Time>) {
    for (electron, mut transform) in query {
        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = electron.speed.0 * time.delta_secs();
        transform.translation += movement_direction * movement_distance;
    }
}

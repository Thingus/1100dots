use bevy::{
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    post_process::bloom::Bloom,
    prelude::*,
    window::PrimaryWindow,
};
use rand::Rng;
use std::f32::consts::PI;
const ELECTRON_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const ELECTRON_SIZE: f32 = 3.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                spawn_electrons,
                move_electrons,
                influence_electrons,
                move_influencer,
            ),
        )
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
struct HeldInfluencer;

#[derive(Component)]
struct ElectronEmitter {
    cone_half_angle: f32,
}

#[derive(Component)]
struct ElectronCollector;

#[derive(Component)]
struct Collidable;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let emitter_mesh = meshes.add(Triangle2d::new(
        Vec2::Y * 10.0,
        Vec2::new(-10.0, -10.0),
        Vec2::new(10.0, -10.0),
    ));

    let emitter_material = materials.add(Color::WHITE);

    let influencer_mesh = meshes.add(Circle::new(10.));

    let influencer_material = materials.add(Color::WHITE);

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
        ElectronEmitter {
            cone_half_angle: 0.1 * PI,
        },
        Transform {
            translation: Vec3::new(-300., -30., 2.),
            ..default()
        },
        Mesh2d(emitter_mesh),
        MeshMaterial2d(emitter_material),
    ));

    commands.spawn((
        ElectronInfluencer {
            radius: 50.,
            magnitude: 1.,
        },
        Transform {
            translation: Vec3::new(-240., 0., 2.),
            ..default()
        },
        Mesh2d(influencer_mesh),
        MeshMaterial2d(influencer_material),
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
        let angle = rng.random_range(-emitter.cone_half_angle..emitter.cone_half_angle);
        let speed = rng.random_range(5.0..100.0);
        commands.spawn((
            Electron {
                speed: Speed(speed),
            },
            emitter_transform
                .with_scale(Vec2::splat(ELECTRON_SIZE).extend(1.))
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

fn move_influencer(
    mut influencer: Single<&mut Transform, With<HeldInfluencer>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = window.cursor_position() {
        influencer.translation = position.extend(3.)
    }
}

fn influence_electrons(
    electron_position: Query<&mut Transform, (With<Electron>, Without<ElectronInfluencer>)>,
    influencers: Query<(&ElectronInfluencer, &Transform), Without<Electron>>,
) {
    for mut electron_tf in electron_position {
        for (influencer, influencer_tf) in influencers {
            if electron_tf.translation.distance(influencer_tf.translation) <= influencer.radius {
                electron_tf.rotate_z(influencer.magnitude);
            }
        }
    }
}

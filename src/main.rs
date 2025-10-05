use bevy::{
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    post_process::bloom::Bloom,
    prelude::*,
    window::PrimaryWindow,
};
use rand::prelude::*;
use std::f32::consts::PI;
const ELECTRON_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const ELECTRON_SIZE: f32 = 3.;
const HOOVER_ROT_SPEED: f32 = PI;

// fn rand_range(start: f32, end: f32) -> f32 {
//     let max = end - start;
//     let offset = rand64() / u64::MAX;
//     start + (offset * max as f32)
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup,
                setup_collector,
                setup_emitter,
                // setup_influencer,
                setup_hoover,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                spawn_electrons,
                move_electrons,
                influence_electrons,
                move_held,
                collect_electrons,
                hoover_electrons,
                rotate_hoover,
                wobble_wobblers,
            ),
        )
        .add_observer(on_electron_collected)
        .init_state::<Levels>()
        .add_systems(OnEnter(Levels::Level2), setup_influencer)
        .add_systems(OnEnter(Levels::Level3), wobble_influencer)
        .add_systems(OnEnter(Levels::Level4), wobble_goal)
        .add_systems(OnEnter(Levels::Level5), wobble_source)
        .add_systems(OnEnter(Levels::Victory), victory_screen)
        .run();
}

fn wobble_influencer(influencers: Query<Entity, With<ElectronInfluencer>>, mut commands: Commands) {
    for influencer in influencers {
        commands.entity(influencer).insert(Wobbler);
    }
}

fn wobble_goal(influencers: Query<Entity, With<ElectronCollector>>, mut commands: Commands) {
    for influencer in influencers {
        commands.entity(influencer).insert(Wobbler);
    }
}

fn wobble_source(influencers: Query<Entity, With<ElectronEmitter>>, mut commands: Commands) {
    for influencer in influencers {
        commands.entity(influencer).insert(Wobbler);
    }
}

fn victory_screen(mut commands: Commands, time: Res<Time>) {
    commands.spawn((
        Text::new("YOU WIN!"),
        Node {
            position_type: PositionType::Absolute,
            top: px(500),
            left: px(500),
            ..default()
        },
    ));
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum Levels {
    #[default]
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Victory,
}

#[derive(Event)]
struct ElectronCollected;

fn on_electron_collected(
    _collected: On<ElectronCollected>,
    mut score: Single<&mut Score>,
    mut text: Single<&mut Text>,
    mut next_state: ResMut<NextState<Levels>>,
) {
    score.0 += 1;
    text.0 = score.0.to_string();

    match score.0 {
        300 => next_state.set(Levels::Level2),
        500 => next_state.set(Levels::Level3),
        700 => next_state.set(Levels::Level4),
        900 => next_state.set(Levels::Level5),
        1100 => next_state.set(Levels::Victory),
        _ => {}
    }
}

#[derive(Component, Default)]
struct Score(i32);

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
struct Wobbler;

#[derive(Component)]
struct ElectronHoover {
    radius: f32,
    magnitude: f32,
    collection_half_angle: f32,
}

#[derive(Component)]
struct Held;

#[derive(Component)]
struct ElectronEmitter {
    cone_half_angle: f32,
}

#[derive(Component)]
struct ElectronCollector {
    radius: f32,
}

#[derive(Component)]
struct Collidable;

fn setup(mut commands: Commands) {
    info!("Camera setup");
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.75,
            ..default()
        },
        DebandDither::Enabled,
    ));

    info!("Score setup");
    commands.spawn(Score(0));

    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn setup_emitter(
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
}
fn setup_influencer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let influencer_mesh = meshes.add(Circle::new(10.));

    let influencer_material = materials.add(Color::WHITE);

    commands.spawn((
        ElectronInfluencer {
            radius: 150.,
            magnitude: 1.,
        },
        Transform {
            translation: Vec3::new(-240., 0., 2.),
            rotation: Quat::from_rotation_z(PI * 0.5),
            ..default()
        },
        Mesh2d(influencer_mesh),
        MeshMaterial2d(influencer_material),
    ));
}
fn setup_hoover(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let hoover_mesh = meshes.add(Triangle2d::new(
        Vec2::Y * 20.0,
        Vec2::new(-10.0, -10.0),
        Vec2::new(10.0, -10.0),
    ));

    let hoover_material = materials.add(Color::WHITE);

    commands.spawn((
        ElectronHoover {
            radius: 150.,
            magnitude: 2.,
            collection_half_angle: 0.25 * PI,
        },
        Transform {
            translation: Vec3::new(-240., 0., 2.),
            ..default()
        },
        Mesh2d(hoover_mesh),
        MeshMaterial2d(hoover_material),
        Held,
    ));
}

fn setup_collector(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let collector_mesh = meshes.add(Circle::new(10.));

    let collector_material = materials.add(Color::srgb(0., 0., 1.));

    commands.spawn((
        ElectronCollector { radius: 50. },
        Transform {
            translation: Vec3::new(200., 0., 2.),
            ..default()
        },
        Mesh2d(collector_mesh),
        MeshMaterial2d(collector_material),
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
        let spray_angle = rng.random_range(-emitter.cone_half_angle..emitter.cone_half_angle);
        let emitter_angle = emitter_transform.rotation.to_scaled_axis().z;
        let angle = spray_angle + emitter_angle;
        // I don't think gimbal lock is real, I think it's a conspiracy by Big Axis to upset me,
        // specifically.

        //using time as a prng
        let speed = rng.random_range(75.0..100.0);
        commands.spawn((
            Electron {
                speed: Speed(speed),
            },
            emitter_transform
                .with_scale(Vec2::splat(ELECTRON_SIZE).extend(1.))
                .with_rotation(Quat::from_scaled_axis(Vec3::new(0., 0., angle))),
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

fn move_held(
    mut influencer: Single<&mut Transform, With<Held>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_bits: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_bits.into_inner();

    if let Some(position) = window
        .cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        influencer.translation = position.extend(3.);
    }
}

fn influence_electrons(
    electron_positions: Query<&mut Transform, (With<Electron>, Without<ElectronInfluencer>)>,
    influencers: Query<(&ElectronInfluencer, &Transform), Without<Electron>>,
    time: Res<Time>,
) {
    for mut electron_tf in electron_positions {
        for (influencer, influencer_tf) in influencers {
            if electron_tf.translation.distance(influencer_tf.translation) <= influencer.radius {
                let (max_angle, rotation_sign) = angle_from_y(&electron_tf, influencer_tf);
                let rotation_angle =
                    rotation_sign * (influencer.magnitude * time.delta_secs()).min(max_angle);
                electron_tf.rotate_z(rotation_angle);
            }
        }
    }
}

// returns the maxiumum angle and direction needed to face electron_tf towards influencer_tf
fn angle_from_y(electron_tf: &Transform, influencer_tf: &Transform) -> (f32, f32) {
    let electron_forward = (electron_tf.rotation * Vec3::Y).xy();
    let to_influencer = (influencer_tf.translation - electron_tf.translation)
        .xy()
        .normalize();
    let forward_dot_influencer = electron_forward.dot(to_influencer);
    let electron_right = (electron_tf.rotation * Vec3::X).xy();
    let right_dot_influencer = electron_right.dot(to_influencer);
    let rotation_sign = -f32::copysign(1.0, right_dot_influencer);
    let max_angle = ops::acos(forward_dot_influencer.clamp(-1., 1.));
    (max_angle, rotation_sign)
}

fn is_clockwise(start: Vec2, end: Vec2) -> bool {
    -start.x * end.y + start.y * end.x > 0.
}

fn is_within_rad(p1: Vec2, p2: Vec2, radius: f32) -> bool {
    p1.distance_squared(p2) < f32::powi(radius, 2)
}

fn is_within_segment(
    point: Vec2,
    center: Vec2,
    sector_start_angle: f32,
    sector_end_angle: f32,
    radius: f32,
) -> bool {
    let sector_start_vec = Vec2::new(
        ops::cos(sector_start_angle + (PI / 2.)),
        ops::sin(sector_start_angle + (PI / 2.)),
    )
    .normalize();
    let sector_end_vec = Vec2::new(
        ops::cos(sector_end_angle + (PI / 2.)),
        ops::sin(sector_end_angle + (PI / 2.)),
    )
    .normalize();
    let rel_vec = point - center;
    is_clockwise(sector_end_vec, rel_vec)
        && !is_clockwise(sector_start_vec, rel_vec)
        && is_within_rad(point, center, radius)
}

fn wobble_wobblers(wobblies: Query<&mut Transform, With<Wobbler>>, time: Res<Time>) {
    for mut wobblie in wobblies {
        let offset = (ops::sin(time.elapsed_secs()) * 3.);
        wobblie.translation.y += offset;
    }
}

fn rotate_hoover(
    keys: Res<ButtonInput<KeyCode>>,
    hoovers: Query<&mut Transform, With<ElectronHoover>>,
    time: Res<Time>,
) {
    for mut hoover in hoovers {
        if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
            hoover.rotate_z(HOOVER_ROT_SPEED * time.delta_secs());
        }
        if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
            hoover.rotate_z(HOOVER_ROT_SPEED * time.delta_secs() * -1.);
        }
    }
}

fn hoover_electrons(
    electron_positions: Query<(Entity, &mut Transform), (With<Electron>, Without<ElectronHoover>)>,
    hoovers: Query<(&ElectronHoover, &Transform), Without<Electron>>,
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, mut electron_tf) in electron_positions {
        for (hoover, hoover_tf) in hoovers {
            let electron_point = electron_tf.translation.xy();
            let hoover_point = hoover_tf.translation.xy();
            let hoover_facing_angle = hoover_tf.rotation.to_scaled_axis().z;
            let sector_start_angle = hoover_facing_angle + hoover.collection_half_angle;
            let sector_end_angle = hoover_facing_angle - hoover.collection_half_angle;
            // let dbg_start = Vec2::new(ops::cos(sector_start_angle), ops::sin(sector_start_angle));
            // let dbg_end = Vec2::new(ops::cos(sector_end_angle), ops::sin(sector_end_angle));
            //
            // gizmos.arc_2d(
            //     Isometry2d::new(hoover_point, Rot2::degrees(sector_start_angle)),
            //     sector_start_angle - sector_end_angle,
            //     hoover.radius,
            //     LIGHT_GREEN,
            // );

            if is_within_segment(
                electron_point,
                hoover_point,
                sector_start_angle,
                sector_end_angle,
                hoover.radius,
            ) {
                let (max_angle, rotation_sign) = angle_from_y(&electron_tf, hoover_tf);
                let rotation_angle =
                    rotation_sign * (hoover.magnitude * time.delta_secs()).min(max_angle);
                electron_tf.rotate_z(rotation_angle);
                commands
                    .entity(entity)
                    .insert(MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))));
            }
            if is_within_rad(electron_point, hoover_point, 1.) {
                electron_tf.rotation = hoover_tf.rotation;
            }
        }
    }
}

fn collect_electrons(
    electron_position: Query<
        (Entity, &mut Transform),
        (With<Electron>, Without<ElectronCollector>),
    >,
    collectors: Query<(&ElectronCollector, &Transform), Without<Electron>>,

    mut commands: Commands,
) {
    for (entity, mut electron_tf) in electron_position {
        for (collector, collector_tf) in collectors {
            if electron_tf.translation.distance(collector_tf.translation) <= collector.radius {
                let to_collector = (collector_tf.translation - electron_tf.translation)
                    .xy()
                    .normalize();
                let rotate_to_collector = Quat::from_rotation_arc(Vec3::Y, to_collector.extend(0.));
                electron_tf.rotation = rotate_to_collector;
            }
            if electron_tf.translation.distance(collector_tf.translation) <= 1. {
                commands.entity(entity).despawn();
                commands.trigger(ElectronCollected);
            }
        }
    }
}

use std::time::Duration;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{
    components::{Bird, Carnivore, Energy, Herb, HerbSpawnConfig, Herbivore},
    BIRDS_TO_SPAWN, BOUNDS,
};

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },

            ..default()
        },
        BloomSettings::default(),
    ));

    let mesh_handle = meshes.add(shape::RegularPolygon::new(10., 3).into());
    let material_handle = materials.add(ColorMaterial::from(Color::hsl(180., 1.0, 0.5)));

    // spawn herbivores
    let batch = (0..BIRDS_TO_SPAWN).map(move |_| {
        let mut rng = rand::thread_rng();
        let random_rotation = Quat::from_rotation_z(rng.gen_range(0.0..std::f32::consts::PI * 2.0));
        (
            MaterialMesh2dBundle {
                mesh: Handle::clone(&mesh_handle).into(),
                material: Handle::clone(&material_handle),
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(-BOUNDS.x / 2.0..BOUNDS.x / 2.0),
                        rng.gen_range(-BOUNDS.y / 2.0..BOUNDS.y / 2.0),
                        0.,
                    ),
                    rotation: random_rotation,
                    ..default()
                },
                ..default()
            },
            Bird {
                speed: 100.,
                rotation_speed: 1.,
                vision_range: 100.,
                desired_direction: random_rotation * Vec3::Y,
            },
            Herbivore,
            Energy {
                value: 50.,
                max: 100.,
            },
        )
    });
    commands.spawn_batch(batch);

    let herb_mesh_handle = meshes.add(shape::Circle::new(3.).into());
    let herb_material_handle = materials.add(ColorMaterial::from(Color::hsl(120., 1.0, 0.5)));
    // spawn herbs
    let batch = (0..100).map(move |_| {
        let mut rng = rand::thread_rng();
        (
            MaterialMesh2dBundle {
                mesh: Handle::clone(&herb_mesh_handle).into(),
                material: Handle::clone(&herb_material_handle),
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(-BOUNDS.x / 2.0..BOUNDS.x / 2.0),
                        rng.gen_range(-BOUNDS.y / 2.0..BOUNDS.y / 2.0),
                        0.,
                    ),
                    ..default()
                },
                ..default()
            },
            Herb { value: 10. },
        )
    });
    commands.spawn_batch(batch);

    // spawn carnivores
    let mut rng = rand::thread_rng();
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(10., 3).into()).into(),
            material: materials.add(ColorMaterial::from(Color::hsl(340., 1.0, 0.5))),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        })
        .insert(Bird {
            speed: 100.,
            rotation_speed: 1.5,
            vision_range: 150.,
            desired_direction: Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.),
        })
        .insert(Carnivore)
        .insert(Energy {
            value: 50.,
            max: 100.,
        });
}

pub fn energy_drain(time: Res<Time>, mut query: Query<(&mut Energy, &Bird)>) {
    query.iter_mut().for_each(|(mut energy, bird)| {
        energy.value -= time.delta_seconds() * bird.speed / 100.;
    });
}

pub fn zero_energy_dies(mut commands: Commands, query: Query<(Entity, &Energy)>) {
    query.for_each(|(entity, energy)| {
        if energy.value <= 0. {
            commands.entity(entity).despawn();
        }
    });
}

pub fn spawn_herbivore_offspring(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Energy, &Bird, &Transform), With<Herbivore>>,
) {
    query.iter_mut().for_each(|(mut energy, bird, transform)| {
        if energy.value >= energy.max * 0.8 {
            energy.value /= 2.;

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::RegularPolygon::new(10., 3).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::hsl(180., 1.0, 0.5))),
                    transform: Transform {
                        translation: transform.translation - transform.rotation * Vec3::Y * 10.,
                        rotation: transform.rotation,
                        ..default()
                    },
                    ..default()
                },
                *bird,
                Herbivore,
                Energy {
                    value: 50.,
                    max: 100.,
                },
            ));
        }
    });
}

pub fn spawn_carnivore_offspring(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Energy, &Bird, &Transform), With<Carnivore>>,
) {
    query.iter_mut().for_each(|(mut energy, bird, transform)| {
        if energy.value >= energy.max * 0.8 {
            energy.value /= 2.;

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::RegularPolygon::new(10., 3).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::hsl(340., 1.0, 0.5))),
                    transform: Transform {
                        translation: transform.translation - transform.rotation * Vec3::Y * 10.,
                        rotation: transform.rotation,
                        ..default()
                    },
                    ..default()
                },
                *bird,
                Carnivore,
                *energy,
            ));
        }
    });
}

pub fn setup_herb_spawner(mut commands: Commands) {
    commands.insert_resource(HerbSpawnConfig {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
}

pub fn spawn_herbs(
    time: Res<Time>,
    mut commands: Commands,
    mut herb_spaw_config: ResMut<HerbSpawnConfig>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    herb_spaw_config
        .timer
        .tick(Duration::from_secs_f32(time.delta_seconds()));

    if herb_spaw_config.timer.just_finished() {
        let mut rng = rand::thread_rng();
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(3.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::hsl(120., 1.0, 0.5))),
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(-BOUNDS.x / 2.0..BOUNDS.x / 2.0),
                        rng.gen_range(-BOUNDS.y / 2.0..BOUNDS.y / 2.0),
                        0.,
                    ),
                    ..default()
                },
                ..default()
            },
            Herb { value: 10. },
        ));
    }
}

pub fn draw_gizmos(mut gizmos: Gizmos, birds: Query<(&Transform, &Bird)>) {
    gizmos.rect(
        Vec3::ZERO,
        Quat::IDENTITY,
        Vec2::new(BOUNDS.x, BOUNDS.y),
        Color::rgba(0.5, 1., 0.5, 0.5),
    );

    birds.iter().for_each(|(transform, bird)| {
        gizmos.circle(
            transform.translation,
            Vec3::Z,
            bird.vision_range,
            Color::rgba(0., 1., 0., 0.1),
        );
        gizmos.line(
            transform.translation,
            transform.translation + bird.desired_direction.normalize_or_zero() * 100.,
            Color::rgba(1., 0., 0., 0.3),
        );
    });
}

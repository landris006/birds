use bevy::{core_pipeline::bloom::BloomSettings, prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{
    components::{Bird, Carnivore, Energy, Herbivore},
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
        // BloomSettings::default(),
    ));

    let mesh_handle = meshes.add(shape::RegularPolygon::new(10., 3).into());
    let material_handle = materials.add(ColorMaterial::from(Color::hsl(180., 1.0, 0.5)));

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
                value: 100.,
                max: 100.,
            },
        )
    });
    commands.spawn_batch(batch);

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
            value: 100.,
            max: 100.,
        });
}

pub fn energy_drain(time: Res<Time>, mut query: Query<(&mut Energy, &Bird)>) {
    query.iter_mut().for_each(|(mut energy, bird)| {
        energy.value -= time.delta_seconds() * bird.speed / 100.;

        dbg!(energy);
    });
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

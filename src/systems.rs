use bevy::{core_pipeline::bloom::BloomSettings, prelude::*, sprite::MaterialMesh2dBundle};
const BOUNDS: Vec2 = Vec2::new(1800., 900.);
const BIRDS_TO_SPAWN: i32 = 2000;

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

    let batch = (0..BIRDS_TO_SPAWN).map(move |i| {
        (
            MaterialMesh2dBundle {
                mesh: Handle::clone(&mesh_handle).into(),
                material: Handle::clone(&material_handle),
                transform: Transform::from_translation(Vec3::new(
                    (i as f32 / BIRDS_TO_SPAWN as f32) * BOUNDS.x - BOUNDS.x / 2.,
                    (i as f32 / BIRDS_TO_SPAWN as f32) * BOUNDS.x - BOUNDS.x / 2.,
                    0.,
                )),
                ..default()
            },
            Bird {
                speed: 100.,
                rotation_speed: 1.,
                vision_range: 100.,
            },
        )
    });

    commands.spawn_batch(batch);
}

pub fn move_birds(time: Res<Time>, mut query: Query<(&mut Transform, &Bird)>) {
    query.iter_mut().for_each(|(mut transform, bird)| {
        let movement_direction = transform.rotation * Vec3::Y;
        transform.translation.x += movement_direction.x * time.delta_seconds() * bird.speed;
        transform.translation.y += movement_direction.y * time.delta_seconds() * bird.speed;
    });
}

pub fn draw_gizmos(mut gizmos: Gizmos) {
    gizmos.rect(
        Vec3::ZERO,
        Quat::IDENTITY,
        Vec2::new(BOUNDS.x, BOUNDS.y),
        Color::rgba(0.5, 1., 0.5, 0.5),
    );
}

pub fn rotate_birds(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Bird)>,
    mut gizmos: Gizmos,
) {
    let birds = query.iter().map(|(e, t, _)| (e, *t)).collect::<Vec<_>>();

    query.iter_mut().for_each(|(e, mut transform, bird)| {
        let birds_in_vision_range = birds
            .iter()
            .filter(|(other_e, _)| *other_e != e)
            .map(|(_, other_transform)| {
                let distance = (other_transform.translation - transform.translation).length();
                (distance, other_transform)
            })
            .filter(|(distance, _)| *distance < bird.vision_range)
            .collect::<Vec<_>>();

        let mut desired_direction = transform.rotation * Vec3::Y;

        let is_out_of_bounds = transform.translation.x.abs() >= BOUNDS.x / 2.0
            || transform.translation.y.abs() >= BOUNDS.y / 2.0;
        if is_out_of_bounds || birds_in_vision_range.is_empty() {
            let to_center = Vec3::ZERO - transform.translation;
            desired_direction += 10. * to_center.normalize_or_zero();
        }

        let separation = birds_in_vision_range
            .iter()
            .filter(|(distance, _)| *distance < 20.)
            .map(|(_, other_transform)| other_transform.translation - transform.translation)
            .sum::<Vec3>();
        let cohesion = birds_in_vision_range
            .iter()
            .map(|(_, other_transform)| other_transform.translation - transform.translation)
            .sum::<Vec3>();
        let alignment = birds_in_vision_range
            .iter()
            .map(|(_, other_transform)| other_transform.rotation * Vec3::Y)
            .sum::<Vec3>();

        desired_direction += alignment.normalize_or_zero()
            + 0.1 * cohesion.normalize_or_zero()
            + 2.0 * -separation.normalize_or_zero();

        gizmos.circle(
            transform.translation,
            Vec3::Z,
            bird.vision_range,
            if separation != Vec3::ZERO {
                Color::rgba(1., 0., 0., 0.1)
            } else {
                Color::rgba(0., 1., 0., 0.1)
            },
        );
        gizmos.line(
            transform.translation,
            transform.translation + desired_direction * 10.,
            Color::rgba(1., 0., 0., 0.3),
        );

        if desired_direction != Vec3::ZERO {
            transform.rotation = transform.rotation.lerp(
                Quat::from_rotation_arc(Vec3::Y, desired_direction.normalize()),
                bird.rotation_speed * time.delta_seconds(),
            );
        }
    });
}

#[derive(Component)]
pub struct Bird {
    speed: f32,
    rotation_speed: f32,
    vision_range: f32,
}

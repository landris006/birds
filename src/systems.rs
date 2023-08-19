use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
const BOUNDS: Vec2 = Vec2::new(800., 400.);
const BIRDS_TO_SPAWN: i32 = 2000;

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..BIRDS_TO_SPAWN {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(10., 3).into()).into(),
                transform: Transform::from_rotation(Quat::from_rotation_z(0.5 * i as f32)),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                ..default()
            })
            .insert(Bird {
                speed: 75.,
                rotation_speed: 2.,
            });
    }
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

pub fn rotate_birds(time: Res<Time>, mut query: Query<(Entity, &mut Transform, &Bird)>) {
    let birds = query
        .iter_mut()
        .map(|(e, t, _)| (e, *t))
        .collect::<Vec<_>>();

    query.iter_mut().for_each(|(e, mut transform, bird)| {
        let is_out_of_bounds = transform.translation.x.abs() >= BOUNDS.x / 2.0
            || transform.translation.y.abs() >= BOUNDS.y / 2.0;
        if is_out_of_bounds {
            let to_center = (Vec3::ZERO - transform.translation).normalize();
            let new_rotation = Quat::from_rotation_arc(Vec3::Y, to_center);

            transform.rotation = transform
                .rotation
                .lerp(new_rotation, bird.rotation_speed * time.delta_seconds());
            return;
        }

        let mut other_birds = birds
            .iter()
            .filter(|(other_e, _)| *other_e != e)
            .map(|(_, other_transform)| {
                let distance = (other_transform.translation - transform.translation).length();
                (distance, other_transform.rotation)
            })
            .collect::<Vec<_>>();
        other_birds.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let closest = other_birds.first().unwrap();
        if closest.0 < 5. {
            let new_rotation = Quat::from_rotation_arc(Vec3::Y, -(closest.1 * Vec3::Y));

            transform.rotation = transform
                .rotation
                .lerp(new_rotation, bird.rotation_speed * time.delta_seconds());
            return;
        }

        let closest = other_birds
            .iter()
            .take(50)
            .map(|(_, rotation)| *rotation * Vec3::Y)
            .sum::<Vec3>();

        let new_rotation = Quat::from_rotation_arc(Vec3::Y, closest.normalize());
        transform.rotation = transform
            .rotation
            .lerp(new_rotation, bird.rotation_speed * time.delta_seconds());
    });
}

#[derive(Component)]
pub struct Bird {
    speed: f32,
    rotation_speed: f32,
}

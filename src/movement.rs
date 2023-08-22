use crate::{
    components::{Bird, Carnivore, Energy, Herb, Herbivore},
    BOUNDS,
};
use bevy::prelude::*;

pub fn move_birds_forward(time: Res<Time>, mut query: Query<(&mut Transform, &mut Bird)>) {
    query.iter_mut().for_each(|(mut transform, mut bird)| {
        bird.desired_direction = bird.desired_direction.normalize_or_zero();

        if bird.desired_direction != Vec3::ZERO {
            transform.rotation = transform.rotation.lerp(
                Quat::from_rotation_arc(Vec3::Y, bird.desired_direction),
                bird.rotation_speed * time.delta_seconds(),
            );
        }

        let movement_direction = transform.rotation * Vec3::Y;
        transform.translation.x += movement_direction.x * time.delta_seconds() * bird.speed;
        transform.translation.y += movement_direction.y * time.delta_seconds() * bird.speed;
    });
}

pub fn herbivore_flock_movement(
    mut herbivore_query: Query<(Entity, &Transform, &mut Bird), With<Herbivore>>,
) {
    let herbivores = herbivore_query
        .iter()
        .map(|(e, t, h)| (e, *t, *h))
        .collect::<Vec<_>>();

    herbivore_query
        .iter_mut()
        .for_each(|(e, transform, mut bird)| {
            let herbivores_in_vision_range = herbivores
                .iter()
                .filter(|(other_e, _, _)| *other_e != e)
                .map(|(_, other_transform, _)| {
                    let distance = (other_transform.translation - transform.translation).length();
                    (distance, other_transform)
                })
                .filter(|(distance, _)| *distance < bird.vision_range)
                .collect::<Vec<_>>();

            if herbivores_in_vision_range.is_empty() {
                return;
            }

            let separation = herbivores_in_vision_range
                .iter()
                .filter(|(distance, _)| *distance < 20.)
                .map(|(_, other_transform)| other_transform.translation - transform.translation)
                .sum::<Vec3>();

            let (cohesion, alignment) = herbivores_in_vision_range
                .iter()
                .map(|(_, other_transform)| {
                    (
                        other_transform.translation - transform.translation,
                        other_transform.rotation * Vec3::Y,
                    )
                })
                .fold((Vec3::ZERO, Vec3::ZERO), |(sum, sum_rot), (pos, rot)| {
                    (sum + pos, sum_rot + rot)
                });

            bird.desired_direction += alignment.normalize_or_zero()
                + 0.1 * cohesion.normalize_or_zero()
                + 5.0 * -separation.normalize_or_zero();
        });
}

pub fn herbivore_feed(
    mut commands: Commands,
    mut herbivore_query: Query<(&Transform, &mut Bird, &mut Energy), With<Herbivore>>,
    herb_query: Query<(Entity, &Transform, &Herb)>,
) {
    herbivore_query
        .iter_mut()
        .for_each(|(transform, mut bird, mut energy)| {
            if energy.value >= energy.max * 0.95 {
                return;
            }

            let closest_herb_in_vision_range = herb_query
                .iter()
                .map(|(herb_entity, herb_transform, herb)| {
                    let distance = (herb_transform.translation - transform.translation).length();
                    (herb_entity, herb_transform, herb, distance)
                })
                .filter(|(_, _, _, distance)| *distance < bird.vision_range)
                .min_by(|(_, _, _, distance_a), (_, _, _, distance_b)| {
                    distance_a.partial_cmp(distance_b).unwrap()
                });

            if let Some((herb_entity, herb_transform, herb, distance)) =
                closest_herb_in_vision_range
            {
                bird.desired_direction +=
                    2. * (herb_transform.translation - transform.translation).normalize_or_zero();

                if distance < 20. {
                    energy.value += herb.value;
                    energy.value = energy.value.min(energy.max);

                    commands.get_entity(herb_entity).unwrap().despawn();
                }
            }
        });
}

pub fn herbivore_flee(
    mut herbivore_query: Query<(&Transform, &mut Bird), With<Herbivore>>,
    carnivore_query: Query<&Transform, With<Carnivore>>,
) {
    herbivore_query
        .iter_mut()
        .for_each(|(transform, mut bird)| {
            let closest_carnivore_in_vision_range = carnivore_query
                .iter()
                .map(|other_transform| {
                    let distance = (other_transform.translation - transform.translation).length();
                    (other_transform, distance)
                })
                .filter(|(_, distance)| *distance < bird.vision_range)
                .min_by(|(_, distance_a), (_, distance_b)| {
                    distance_a.partial_cmp(distance_b).unwrap()
                });

            if let Some((carnivore_transform, _)) = closest_carnivore_in_vision_range {
                bird.desired_direction += -10.
                    * (carnivore_transform.translation - transform.translation).normalize_or_zero();
                bird.speed = 150.;
            } else {
                bird.speed = 100.;
            }
        })
}

pub fn carnivore_movement(
    mut commands: Commands,
    mut carnivore_query: Query<(&Transform, &mut Bird, &mut Energy), With<Carnivore>>,
    herbivore_query: Query<(Entity, &Transform), With<Herbivore>>,
) {
    carnivore_query
        .iter_mut()
        .for_each(|(transform, mut carnivore, mut energy)| {
            if energy.value >= energy.max * 0.95 {
                carnivore.speed = 100.;
                return;
            }

            let closest_herbivore_in_vision_range = herbivore_query
                .iter()
                .map(|(e, other_transform)| {
                    let distance = (other_transform.translation - transform.translation).length();
                    (e, other_transform, distance)
                })
                .filter(|(_, _, distance)| *distance < carnivore.vision_range)
                .min_by(|(_, _, distance_a), (_, _, distance_b)| {
                    distance_a.partial_cmp(distance_b).unwrap()
                });

            if let Some((herbivore_entity, herbivore_transform, distance)) =
                closest_herbivore_in_vision_range
            {
                let predicted_herbivore_position =
                    herbivore_transform.translation + herbivore_transform.rotation * Vec3::Y * 10.;
                carnivore.desired_direction +=
                    (predicted_herbivore_position - transform.translation).normalize_or_zero();

                carnivore.speed = 175.;

                if distance < 20. {
                    energy.value += 10.;
                    energy.value = energy.value.min(energy.max);

                    commands.get_entity(herbivore_entity).unwrap().despawn();
                }
            } else {
                carnivore.speed = 100.;
            }
        })
}

pub fn zero_energy_dies(mut commands: Commands, query: Query<(Entity, &Energy)>) {
    query.for_each(|(entity, energy)| {
        if energy.value <= 0. {
            commands.entity(entity).despawn();
        }
    });
}

pub fn keep_birds_in_bounds(mut query: Query<(&Transform, &mut Bird)>) {
    query.iter_mut().for_each(|(transform, mut bird)| {
        let is_out_of_bounds = transform.translation.x.abs() >= BOUNDS.x / 2.0
            || transform.translation.y.abs() >= BOUNDS.y / 2.0;
        if is_out_of_bounds {
            let to_center = Vec3::ZERO - transform.translation;
            bird.desired_direction += 10. * to_center.normalize_or_zero();
        }
    });
}

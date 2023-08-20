use bevy::prelude::{Component, Vec3};

#[derive(Component, Clone, Copy)]
pub struct Bird {
    pub speed: f32,
    pub rotation_speed: f32,
    pub vision_range: f32,
    pub desired_direction: Vec3,
}

#[derive(Component)]
pub struct Herbivore;
#[derive(Component)]
pub struct Carnivore;

#[derive(Component, Debug)]
pub struct Energy {
    pub value: f32,
    pub max: f32,
}

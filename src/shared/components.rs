use bevy::prelude::{Component, Vec2};

#[derive(Component, Clone, Copy)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone, Copy)]
pub struct RadiusCollider {
    pub radius: f32
}

#[derive(Component)]
pub struct DespawnOffscreen;
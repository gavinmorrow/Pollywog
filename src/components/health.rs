use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub remaining: f32,
    #[allow(dead_code)]
    pub total: f32,
}

impl Health {
    pub fn new(remaining: f32, total: f32) -> Self {
        Health { remaining, total }
    }

    pub fn full(total: f32) -> Self {
        Health::new(total, total)
    }
}

impl Default for Health {
    fn default() -> Self {
        Health::full(100.0)
    }
}

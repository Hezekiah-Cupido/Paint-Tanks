use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Bullet {
    pub damage: u8,
}

impl Bullet {
    pub fn new(damage: u8) -> Self {
        Self { damage }
    }
}

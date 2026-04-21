use bevy::{color::Color, ecs::component::Component};

#[derive(Component)]
pub struct Country {
    pub name: String,
    pub color: Color,
    pub money: usize,
}

#[derive(Component)]
pub struct PlayingCountry;

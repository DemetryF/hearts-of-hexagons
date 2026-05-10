use crate::HexagonPos;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub color: Color,
    pub money: usize,
}

#[derive(Component, Serialize, Deserialize)]
pub struct Province {
    pub pos: HexagonPos,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Owner(#[entities] pub Option<Entity>);

#[derive(Component, Serialize, Deserialize)]
#[require(Replicated)]
pub struct Division {
    pub organization: f32,
    pub max_organization: f32,
    pub hp: f32,
    pub max_hp: f32,

    pub speed: f32,

    pub pos: HexagonPos,
    #[entities]
    pub country: Entity,
}

#[derive(Component, Serialize, Deserialize)]
pub struct CombatStats {
    pub attack: f32,
    pub defend: f32,
    pub breakthrough: f32,
}

#[derive(Component, Serialize, Deserialize)]
pub struct MovingOrder {
    pub to: HexagonPos,
}

#[derive(Component, Serialize, Deserialize)]
pub struct Path {
    pub provs: Vec<HexagonPos>,
    pub progress: usize,
}

#[derive(Component, Default, Serialize, Deserialize)]
pub struct MovementBlock;

#[derive(Component, Serialize, Deserialize)]
#[require(MovementBlock)]
pub struct AttacksOn(pub HexagonPos);

#[derive(Component, Serialize, Deserialize)]
pub struct DefendsFrom(pub HexagonPos);

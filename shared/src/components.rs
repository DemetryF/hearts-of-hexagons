use crate::HexagonPos;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Component, Serialize, Deserialize)]
#[require(Replicated)]
pub struct Country {
    pub name: String,
    pub color: Color,
    pub money: usize,
}

#[derive(Component, Serialize, Deserialize)]
#[require(Replicated)]
pub struct Province {
    pub pos: HexagonPos,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvinceOwner(#[entities] pub Option<Entity>);

#[derive(Component, Serialize, Deserialize)]
#[require(Replicated)]
pub struct Division {
    pub organization: f32,
    pub max_organization: f32,
    pub hp: f32,
    pub max_hp: f32,

    pub speed: f32,
}

#[derive(Component, Serialize, Deserialize)]
pub struct DivisionPos(pub HexagonPos);

#[derive(Component, Serialize, Deserialize)]
#[component(immutable)]
pub struct DivisionOwner(#[entities] pub Entity);

#[derive(Component)]
#[component(immutable)]
pub struct DivisionVisibility(#[entities] pub SmallVec<[Entity; 7]>);

impl VisibilityFilter for DivisionVisibility {
    type ClientComponent = Plays;
    type Scope = Entity;

    fn is_visible(&self, _client: Entity, component: Option<&Self::ClientComponent>) -> bool {
        component.is_some_and(|&Plays(country)| self.0.contains(&country))
    }
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

#[derive(Component)]
#[component(immutable)]
pub struct Plays(pub Entity);

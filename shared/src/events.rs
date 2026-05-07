use crate::HexagonPos;
use bevy::{ecs::entity::MapEntities, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Event, Serialize, Deserialize, MapEntities, Clone)]
pub struct MovingOrder {
    #[entities]
    pub entity: Entity,
    pub to: HexagonPos,
}

#[derive(EntityEvent, Serialize, Deserialize)]
pub struct DivisionMoved {
    pub entity: Entity,

    pub from: HexagonPos,
    pub to: HexagonPos,
}

#[derive(Event, Serialize, Deserialize)]
pub struct CountryAssignment {
    pub entity: Entity,
}

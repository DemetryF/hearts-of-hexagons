use {
    crate::HexagonPos,
    bevy::{ecs::entity::MapEntities, prelude::*},
    serde::{Deserialize, Serialize},
};

#[derive(Event, Serialize, Deserialize, MapEntities, Clone)]
pub struct MovingOrderEvent {
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

#[derive(Event, Serialize, Deserialize, MapEntities, Clone)]
pub struct CountryAssignmentRequest {
    #[entities]
    pub entity: Entity,
}

#[derive(Event, Serialize, Deserialize, MapEntities, Clone)]
pub enum CountryAssignmentResponse {
    Success(#[entities] Entity),
    CountryIsBusy,
}

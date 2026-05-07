use bevy::prelude::*;
use shared::HexagonPos;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct Map {
    pub provs: HashMap<HexagonPos, Entity>,
}

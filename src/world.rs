use std::collections::HashMap;

use bevy::prelude::*;

use crate::hexagon_pos::HexagonPos;

#[derive(Default, Resource, Clone)]
pub struct Map {
    pub provinces: HashMap<HexagonPos, Province>,
}

#[derive(Clone)]
pub struct Province {
    pub control: Option<Entity>,
}

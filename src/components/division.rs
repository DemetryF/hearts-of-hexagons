mod interface;
mod movement;

pub use interface::*;
pub use movement::*;

use bevy::prelude::*;

use crate::{components::Country, hexagon_pos::HexagonPos};

const REGENERATION_COST: usize = 20;

#[derive(Component)]
pub struct Division {
    pub hp: usize,
    pub max_hp: usize,

    pub attack: usize,
    pub defend: usize,
    pub speed: f32,

    pub pos: HexagonPos,
    pub country: Entity,
}

pub fn regenerate_division(divisions: Query<&mut Division>, mut countries: Query<&mut Country>) {
    for mut division in divisions {
        if division.hp == division.max_hp {
            continue;
        }

        let mut country = countries.get_mut(division.country).unwrap();

        let missing_hp = division.max_hp - division.hp;
        let can_regenerate = country.money / REGENERATION_COST;

        let regenerate = usize::min(can_regenerate, missing_hp);
        let regeneration_cost = regenerate * REGENERATION_COST;

        division.hp += regenerate;
        country.money -= regeneration_cost;
    }
}

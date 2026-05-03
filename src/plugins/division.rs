mod division_mesh;
mod division_movement;
mod division_selection;

pub use {division_mesh::*, division_movement::*, division_selection::*};

use {
    crate::{country::Country, hexagon_pos::HexagonPos, plugins::Tick},
    bevy::prelude::*,
};

const REGENERATION_COST: usize = 20;

pub struct DivisionPlugin;

impl Plugin for DivisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Tick, regenerate_division).add_plugins((
            DivisionMeshPlugin,
            DivisionSelectionPlugin,
            DivisionMovementPlugin,
        ));
    }
}

#[derive(Component)]
pub struct Division {
    pub organization: f32,
    pub max_organization: f32,
    pub hp: f32,
    pub max_hp: f32,

    pub speed: f32,

    pub pos: HexagonPos,
    pub country: Entity,
}

#[derive(Component)]
pub struct CombatStats {
    pub attack: f32,
    pub defend: f32,
    pub breakthrough: f32,
}

// limit speed of regeneration
fn regenerate_division(divisions: Query<&mut Division>, mut countries: Query<&mut Country>) {
    for mut division in divisions {
        if division.hp == division.max_hp {
            continue;
        }

        let mut country = countries.get_mut(division.country).unwrap();

        let missing_hp = division.max_hp - division.hp;
        let can_regenerate = country.money / REGENERATION_COST;

        let regenerate = f32::min(can_regenerate as f32, missing_hp);
        let regeneration_cost = (regenerate * REGENERATION_COST as f32) as usize;

        division.hp += regenerate;
        country.money -= regeneration_cost;
    }
}

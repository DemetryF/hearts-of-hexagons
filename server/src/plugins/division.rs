use crate::{
    map::Map,
    plugins::{Tick, trigger_attack},
};
use bevy::prelude::*;
use shared::*;
use smallvec::SmallVec;

const REGENERATION_COST: usize = 50;

pub struct DivisionPlugin;

impl Plugin for DivisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Tick,
            (
                update_division_on_border_with,
                (recovery, regeneration).after(trigger_attack),
            ),
        );
    }
}

fn recovery(divisions: Query<&mut Division, (Without<AttacksOn>, Without<DefendsFrom>)>) {
    for mut division in divisions {
        if division.organization < division.recovery_speed {
            division.organization += division.recovery_speed;
            division.organization = division.organization.min(division.max_organization);
        }
    }
}

fn regeneration(
    divisions: Query<(&mut Division, &DivisionOwner), (Without<AttacksOn>, Without<DefendsFrom>)>,
    mut countries: Query<&mut Country>,
) {
    for (mut division, owner) in divisions {
        let mut country = countries.get_mut(owner.0).unwrap();

        let diff = division.max_hp - division.hp;

        if diff > 0. {
            let can_regenerate = diff.min(1.);
            let cost = (REGENERATION_COST as f32 * can_regenerate).floor() as usize;

            if country.money >= cost {
                country.money -= cost;
                division.hp += can_regenerate;
            }
        }
    }
}

fn update_division_on_border_with(
    changed: Query<(), Changed<DivisionPos>>,
    provs: Query<&Province, Changed<ProvinceOwner>>,
    divisions: Query<(Entity, &DivisionPos, &DivisionOwner)>,
    owners: Query<&ProvinceOwner>,
    map: Res<Map>,
    mut commands: Commands,
) {
    for (entity, &DivisionPos(pos), &DivisionOwner(owner)) in divisions {
        if !(changed.contains(entity)
            || provs.iter().any(|prov| {
                (prov.pos.neighbours())
                    .iter()
                    .any(|&neighbor_pos| neighbor_pos == pos)
            }))
        {
            continue;
        }

        let mut new_neighbours = SmallVec::new();

        new_neighbours.push(owner);

        for neighbor in pos.neighbours() {
            if let Some(&prov) = map.provs.get(&neighbor)
                && let Some(owner) = owners.get(prov).unwrap().0
                && !new_neighbours.contains(&owner)
            {
                new_neighbours.push(owner);
            }
        }

        commands
            .entity(entity)
            .insert(DivisionVisibility(new_neighbours));
    }
}

use crate::{map::Map, plugins::Tick};
use bevy::prelude::*;
use shared::*;
use smallvec::SmallVec;

pub struct DivisionPlugin;

impl Plugin for DivisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Tick, update_division_on_border_with);
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

use std::collections::HashSet;

use bevy::prelude::*;
use bevy_replicon::shared::message::client_event::ClientTriggerExt;
use shared::*;

use crate::plugins::{DivisionsAtProvince, HoveredProvince, SIDE, SelectedDivision};

pub struct DivisionMovementPlugin;

impl Plugin for DivisionMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                create_moving_order,
                (update_prev_pos, init_prev_pos, update_division_mesh).chain(),
            ),
        );
    }
}

fn create_moving_order(
    selected: Option<Single<Entity, With<SelectedDivision>>>,
    hovered_prov: Res<HoveredProvince>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    if input.just_pressed(MouseButton::Left)
        && let Some(hovered) = hovered_prov.0
        && let Some(selected) = selected
    {
        commands.client_trigger(MovingOrder {
            entity: *selected,
            to: hovered,
        });

        commands.entity(*selected).remove::<SelectedDivision>();
    }
}

#[derive(Component)]
pub struct PrevPos {
    actual: HexagonPos,
    pub prev: HexagonPos,
}

fn init_prev_pos(divisions: Query<(Entity, &Division), Added<Division>>, mut commands: Commands) {
    for (entity, div) in divisions {
        commands.entity(entity).insert(PrevPos {
            actual: div.pos,
            prev: div.pos,
        });
    }
}

fn update_prev_pos(divisions: Query<(&Division, &mut PrevPos), Changed<Division>>) {
    for (div, mut prev_pos) in divisions {
        prev_pos.prev = prev_pos.actual;
        prev_pos.actual = div.pos;
    }
}

fn update_division_mesh(
    mut divisions: Query<(&mut Transform, &Division), With<Division>>,
    changed: Query<&PrevPos, Changed<Division>>,
    mut divisions_at_prov: ResMut<DivisionsAtProvince>,
) {
    let affected_provs: HashSet<_> = changed
        .iter()
        .flat_map(|prev| [prev.actual, prev.prev])
        .collect();

    for &prov in &affected_provs {
        divisions_at_prov.0.insert(prov, 0);
    }

    let affected_divisions = divisions
        .iter_mut()
        .filter(|(_, div)| affected_provs.contains(&div.pos));

    for (mut transform, division) in affected_divisions {
        let divisions_at_the_prov = divisions_at_prov.0[&division.pos];

        let pos = division.pos.real_regular(SIDE);
        let shift = Vec2::new(0., 3. * divisions_at_the_prov as f32);

        (divisions_at_prov.0)
            .entry(division.pos)
            .and_modify(|count| *count += 1);

        transform.translation = Vec3::new(pos.x + shift.x, pos.y + shift.y, 1.);
    }
}

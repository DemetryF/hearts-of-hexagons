use {
    crate::{
        common::SIDE,
        game::{DivisionsAtProvince, SelectedDivision},
    },
    bevy::prelude::*,
    bevy_replicon::shared::message::client_event::ClientTriggerExt,
    shared::*,
    std::collections::HashSet,
};

pub struct DivisionMovementPlugin;

impl Plugin for DivisionMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_prev_pos, init_prev_pos, update_division_mesh).chain(),
        )
        .add_observer(create_moving_order);
    }
}

fn create_moving_order(
    event: On<Pointer<Press>>,
    selected: Query<Entity, With<SelectedDivision>>,
    provs: Query<&Province>,
    mut commands: Commands,
) {
    let Ok(dst) = provs.get(event.entity) else {
        return;
    };

    for entity in selected {
        commands.client_trigger(MovingOrderEvent {
            entity,
            to: dst.pos,
        });

        println!("created moving order");

        commands.entity(entity).remove::<SelectedDivision>();
    }
}

#[derive(Component)]
pub struct PrevPos {
    actual: HexagonPos,
    pub prev: HexagonPos,
}

fn init_prev_pos(
    divisions: Query<(Entity, &DivisionPos), Added<DivisionPos>>,
    mut commands: Commands,
) {
    for (entity, &DivisionPos(pos)) in divisions {
        commands.entity(entity).insert(PrevPos {
            actual: pos,
            prev: pos,
        });
    }
}

fn update_prev_pos(divisions: Query<(&DivisionPos, &mut PrevPos), Changed<DivisionPos>>) {
    for (&DivisionPos(pos), mut prev_pos) in divisions {
        prev_pos.prev = prev_pos.actual;
        prev_pos.actual = pos;
    }
}

fn update_division_mesh(
    mut divisions: Query<(&mut Transform, &DivisionPos)>,
    changed: Query<&PrevPos, Changed<DivisionPos>>,
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
        .filter(|(_, pos)| affected_provs.contains(&pos.0));

    for (mut transform, &DivisionPos(pos)) in affected_divisions {
        let divisions_at_the_prov = divisions_at_prov.0[&pos];

        let real_pos = pos.real_regular(SIDE);
        let shift = Vec2::new(0., 3. * divisions_at_the_prov as f32);

        (divisions_at_prov.0)
            .entry(pos)
            .and_modify(|count| *count += 1);

        transform.translation = Vec3::new(real_pos.x + shift.x, real_pos.y + shift.y, 1.);
    }
}

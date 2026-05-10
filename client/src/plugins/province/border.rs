use bevy::prelude::*;
use shared::*;

use crate::plugins::{Map, SIDE, setup_provs_meshes};

pub struct BorderPlugin;

impl Plugin for BorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_borders.after(setup_provs_meshes), border_mesh).chain(),
        );
    }
}

#[derive(Component, PartialEq, Eq)]
pub struct Border {
    between: [HexagonPos; 2],
}

impl Border {
    pub fn new(a: HexagonPos, b: HexagonPos) -> Self {
        Self { between: [a, b] }
    }
}

fn update_borders(
    provs: Query<(&Province, &Owner), Changed<Owner>>,
    owners: Query<&Owner>,
    borders: Query<(Entity, &Border)>,
    map: Res<Map>,
    mut commands: Commands,
) {
    for (prov, owner) in provs {
        for (id, border) in borders {
            if border.between.contains(&prov.pos) {
                commands.entity(id).despawn();
            }
        }

        for neighbor in prov.pos.neighbours() {
            let border = (map.provs.get(&neighbor))
                .is_some_and(|&neighbor| owners.get(neighbor).unwrap() != owner);

            if border {
                commands.spawn(Border::new(prov.pos, neighbor));
            }
        }
    }
}

fn border_mesh(
    borders: Query<(Entity, &Border), Without<Mesh2d>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (id, border) in borders {
        let side = border.between[0].side_between_regular(border.between[1], SIDE);

        commands.entity(id).insert((
            Mesh2d(meshes.add(Segment2d::new(side.0, side.1))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(0., 0., 0.1),
        ));
    }
}

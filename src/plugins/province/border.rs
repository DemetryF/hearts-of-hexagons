use {
    crate::{
        hexagon_pos::HexagonPos,
        plugins::{Map, Owner, Province},
    },
    bevy::prelude::*,
    std::iter::zip,
};

pub struct BorderPlugin;

impl Plugin for BorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_borders,));
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

pub fn update_borders(
    provs: Query<(&Province, &Owner), Changed<Owner>>,
    owners: Query<&Owner>,
    borders: Query<(Entity, &Border)>,
    map: Res<Map>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (prov, owner) in provs {
        for (id, border) in borders {
            if border.between.contains(&prov.pos) {
                commands.entity(id).despawn();
            }
        }

        let neighbors = zip(prov.pos.neighbours(), prov.pos.sides_regular(5.));

        for (neighbor, side) in neighbors {
            let border = (map.provs.get(&neighbor))
                .is_some_and(|&neighbor| owners.get(neighbor).unwrap() != owner);

            if border {
                commands.spawn((
                    Border::new(prov.pos, neighbor),
                    Mesh2d(meshes.add(Segment2d::new(side.0, side.1))),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::from_xyz(0., 0., 0.1),
                ));
            }
        }
    }
}

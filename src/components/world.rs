mod interface;

pub use interface::*;

use {
    crate::{components::Country, hexagon_pos::HexagonPos},
    bevy::prelude::*,
    std::{collections::HashMap, f32::consts::PI},
};

const SIDE: f32 = 5.;

#[derive(Default, Resource, Clone)]
pub struct Map {
    pub provs: HashMap<HexagonPos, Entity>,
}

#[derive(Component)]
pub struct Province {
    pub pos: HexagonPos,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Owner(pub Option<Entity>);

#[derive(Component, PartialEq, Eq)]
pub struct Border {
    between: [HexagonPos; 2],
}

impl Border {
    pub fn new(a: HexagonPos, b: HexagonPos) -> Self {
        Self { between: [a, b] }
    }
}

pub fn setup_provs_meshes(
    countries: Query<&Country>,
    provs: Query<(Entity, &Province, &Owner)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let mesh = meshes.add(RegularPolygon::new(SIDE, 6));

    for (id, prov, owner) in provs {
        let pos = prov.pos.real_regular(SIDE);
        let country = owner.0.unwrap();
        let country = countries.get(country).unwrap();

        commands.entity(id).insert((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(country.color)),
            Transform::from_xyz(pos.x, pos.y, 0.0).with_rotation(Quat::from_rotation_z(PI / 2.)),
        ));
    }
}

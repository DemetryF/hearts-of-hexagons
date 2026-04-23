use std::{collections::HashMap, f32::consts::PI};

use bevy::prelude::*;

use crate::{components::Country, hexagon_pos::HexagonPos};

const SIDE: f32 = 5.;

#[derive(Default, Resource, Clone)]
pub struct Map {
    pub provinces: HashMap<HexagonPos, Province>,
}

#[derive(Clone)]
pub struct Province {
    pub control: Option<Entity>,
}

pub fn setup_provinces_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    countries: Query<&Country>,
    map: Res<Map>,
) {
    let mesh = meshes.add(RegularPolygon::new(SIDE, 6));

    for (&hpos, province) in &map.provinces {
        let pos = hpos.real_regular(SIDE);
        let country = province.control.unwrap();
        let country = countries.get(country).unwrap();
        let color = country.color;

        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(pos.x, pos.y, 0.0).with_rotation(Quat::from_rotation_z(PI / 2.)),
        ));

        for (neighbour, side) in hpos.neighbours().into_iter().zip(hpos.sides_regular(SIDE)) {
            let border = map
                .provinces
                .get(&neighbour)
                .is_some_and(|neighbour| neighbour.control != province.control);

            if border {
                commands.spawn((
                    Mesh2d(meshes.add(Segment2d::new(side.0, side.1))),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::default(),
                ));
            }
        }
    }
}

mod border;
mod province_hovering;

pub use {border::*, province_hovering::*};

use {
    crate::{
        country::Country,
        hexagon_pos::HexagonPos,
        plugins::{Division, DivisionMoved},
    },
    bevy::prelude::*,
    std::{collections::HashMap, f32::consts::PI},
};

pub const SIDE: f32 = 5.;

pub struct ProvincePlugin;

impl Plugin for ProvincePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Map>()
            .add_systems(Startup, setup_provs_meshes)
            .add_systems(Update, update_prov_color)
            .add_observer(capture)
            .add_plugins((BorderPlugin, ProvinceHoveringPlugin));
    }
}

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

pub fn capture(
    event: On<DivisionMoved>,
    divisions: Query<&Division>,
    mut provs: Query<&mut Owner>,
    map: ResMut<Map>,
) {
    let country = divisions.get(event.event_target()).unwrap().country;

    let captured_id = map.provs[&event.to];
    let mut captured_owner = provs.get_mut(captured_id).unwrap();

    if captured_owner.0 != Some(country) {
        captured_owner.0 = Some(country);
    }
}

pub fn update_prov_color(
    provs: Query<(Entity, &Owner), Changed<Owner>>,
    countries: Query<&Country>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (id, owner) in provs {
        let color = countries.get(owner.0.unwrap()).unwrap().color;

        commands
            .entity(id)
            .insert(MeshMaterial2d(materials.add(color)));
    }
}

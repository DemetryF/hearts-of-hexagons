mod border;
mod province_hovering;

pub use {border::*, province_hovering::*};

use {
    bevy::prelude::*,
    shared::*,
    std::{collections::HashMap, f32::consts::PI},
};

pub const SIDE: f32 = 5.;

pub struct ProvincePlugin;

impl Plugin for ProvincePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Map>()
            .add_systems(Update, (update_prov_color, setup_provs_meshes))
            .add_plugins((BorderPlugin, ProvinceHoveringPlugin));
    }
}

#[derive(Default, Resource, Clone)]
pub struct Map {
    pub provs: HashMap<HexagonPos, Entity>,
}

pub fn setup_provs_meshes(
    provs: Query<(Entity, &Province, &ProvinceOwner), Without<Mesh2d>>,
    countries: Query<&Country>,
    mut map: ResMut<Map>,
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
            Pickable::default(),
        ));

        map.provs.insert(prov.pos, id);
    }
}

fn update_prov_color(
    provs: Query<(Entity, &ProvinceOwner), Changed<ProvinceOwner>>,
    countries: Query<&Country>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (id, owner) in provs {
        let color = countries
            .get(owner.0.unwrap())
            .map(|country| country.color)
            .unwrap_or(Color::BLACK);

        commands
            .entity(id)
            .insert(MeshMaterial2d(materials.add(color)));
    }
}

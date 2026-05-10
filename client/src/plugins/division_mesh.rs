use crate::plugins::SIDE;
use shared::*;

use {bevy::prelude::*, std::collections::HashMap};

pub struct DivisionMeshPlugin;

impl Plugin for DivisionMeshPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DivisionsAtProvince::default())
            .add_systems(Update, init_division_mesh);
    }
}

#[derive(Resource, Default)]
pub struct DivisionsAtProvince(pub HashMap<HexagonPos, usize>);

fn init_division_mesh(
    divisions: Query<(Entity, &DivisionPos), Without<Mesh2d>>,
    mut divisions_at_prov: ResMut<DivisionsAtProvince>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    let mesh = meshes.add(Rectangle::new(4., 2.5));

    for (id, &DivisionPos(pos)) in divisions {
        println!("inited division mesh");

        let divisions_at_the_prov = (divisions_at_prov.0).get(&pos).copied().unwrap_or_default();

        let shift = Vec2::new(0., 3. * divisions_at_the_prov as f32);

        *divisions_at_prov.0.entry(pos).or_default() += 1;

        let pos = pos.real_regular(SIDE) + shift;

        commands.entity(id).insert((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(Color::linear_rgb(0.2, 0.8, 0.2))),
            Transform::from_xyz(pos.x, pos.y, 1.),
        ));
    }
}

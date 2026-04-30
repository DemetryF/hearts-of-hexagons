use {
    crate::{
        hexagon_pos::HexagonPos,
        plugins::{Division, DivisionMoved, SIDE},
    },
    bevy::prelude::*,
    std::collections::HashMap,
};

pub struct DivisionMeshPlugin;

impl Plugin for DivisionMeshPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DivisionsAtProvince::default())
            .add_systems(Update, init_division_mesh)
            .add_observer(update_division_mesh);
    }
}

#[derive(Resource, Default)]
pub struct DivisionsAtProvince(HashMap<HexagonPos, usize>);

fn init_division_mesh(
    divisions: Query<(Entity, &Division), Without<Mesh2d>>,
    mut divisions_at_prov: ResMut<DivisionsAtProvince>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    let mesh = meshes.add(Rectangle::new(4., 2.5));

    for (id, division) in divisions {
        let divisions_at_the_prov = (divisions_at_prov.0)
            .get(&division.pos)
            .copied()
            .unwrap_or_default();

        let shift = Vec2::new(0., 3. * divisions_at_the_prov as f32);

        *divisions_at_prov.0.entry(division.pos).or_default() += 1;

        let pos = division.pos.real_regular(SIDE) + shift;

        commands.entity(id).insert((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(Color::linear_rgb(0.2, 0.8, 0.2))),
            Transform::from_xyz(pos.x, pos.y, 0.),
        ));
    }
}

fn update_division_mesh(
    event: On<DivisionMoved>,
    divisions: Query<(&Division, &mut Transform)>,
    mut divisions_at_prov: ResMut<DivisionsAtProvince>,
) {
    let affected_divisions = divisions
        .into_iter()
        .filter(|(division, _)| division.pos == event.from || division.pos == event.to);

    divisions_at_prov.0.insert(event.from, 0);
    divisions_at_prov.0.insert(event.to, 0);

    for (division, mut transform) in affected_divisions {
        let divisions_at_the_prov = divisions_at_prov.0[&division.pos];

        let pos = division.pos.real_regular(5.);
        let shift = Vec2::new(0., 3. * divisions_at_the_prov as f32);

        (divisions_at_prov.0)
            .entry(division.pos)
            .and_modify(|count| *count += 1);

        transform.translation = Vec3::new(pos.x + shift.x, pos.y + shift.y, 0.);
    }
}

use {
    crate::{
        components::{Border, Country, Division, DivisionMoved, Map, Owner, Province},
        hexagon_pos::HexagonPos,
    },
    bevy::prelude::*,
    std::iter::zip,
};

const COLOR_CHANGE_FACTOR: f32 = 1.2;

#[derive(Resource, Default)]
pub struct HoveredProvince(pub Option<HexagonPos>);

#[derive(Resource, Default)]
pub struct Highlighted(Option<Entity>);

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
    provs: Query<(&mut MeshMaterial2d<ColorMaterial>, &Owner), Changed<Owner>>,
    countries: Query<&Country>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut material, owner) in provs {
        let color = countries.get(owner.0.unwrap()).unwrap().color;

        material.0 = materials.add(color);
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

pub fn update_hovered(
    mut hovered: ResMut<HoveredProvince>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    map: Res<Map>,
) {
    let (camera, camera_transform) = camera.into_inner();

    let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|c| camera.viewport_to_world_2d(camera_transform, c).ok())
    else {
        hovered.0 = None;
        return;
    };

    let cursor_hpos = HexagonPos::from_real_regular(cursor_pos, 5.);

    if Some(cursor_hpos) == hovered.0 {
        return;
    }

    hovered.0 = map.provs.contains_key(&cursor_hpos).then_some(cursor_hpos);
}

pub fn unhighlight(
    mut provs: Query<(&Owner, &mut MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    countries: Query<&Country>,
    highlighted: ResMut<Highlighted>,
) {
    let Some(prev_id) = highlighted.0 else { return };

    let (owner, mut material) = provs.get_mut(prev_id).unwrap();

    // TODO province can be unclaimed
    let prov_owner = owner.0.unwrap();
    let country = countries.get(prov_owner).unwrap();
    let color = country.color;

    *material = MeshMaterial2d(materials.add(color));
}

pub fn update_highlighted(
    mut provs: Query<(Entity, &Province, &mut MeshMaterial2d<ColorMaterial>), With<Mesh2d>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut highlighted: ResMut<Highlighted>,
    hovered: Res<HoveredProvince>,
) {
    let Some(hovered_pos) = hovered.0 else {
        highlighted.0 = None;
        return;
    };

    let hovered_prov = provs
        .iter_mut()
        .find(|&(_, prov, _)| prov.pos == hovered_pos);

    if let Some((id, _, mut material)) = hovered_prov {
        let color = materials.get(material.id()).unwrap().color;
        let color = multiple(color.to_linear(), COLOR_CHANGE_FACTOR);
        let color = Color::LinearRgba(color);

        *material = MeshMaterial2d(materials.add(color));

        highlighted.0 = Some(id);
    }
}

fn multiple(mut color: LinearRgba, x: f32) -> LinearRgba {
    color.red *= x;
    color.green *= x;
    color.blue *= x;
    color
}

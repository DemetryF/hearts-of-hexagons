use {
    crate::{
        components::{Border, Country, Division, DivisionMoved, Map},
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
    provs: Query<(&HexagonPos, &mut MeshMaterial2d<ColorMaterial>)>,
    countries: Query<&Country>,
    borders: Query<(Entity, &Border)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut map: ResMut<Map>,
    mut commands: Commands,
) {
    let division_country_id = divisions.get(event.event_target()).unwrap().country;
    let division_country = countries.get(division_country_id).unwrap();

    if map.provinces[&event.to].control != Some(division_country_id) {
        map.provinces.get_mut(&event.to).unwrap().control = Some(division_country_id);

        let (_, mut material) = provs.into_iter().find(|&(&p, _)| p == event.to).unwrap();

        material.0 = materials.add(division_country.color);

        for (id, _) in borders
            .into_iter()
            .filter(|(_, b)| b.between.contains(&event.to))
        {
            commands.entity(id).despawn();
        }

        for (neighbour, side) in zip(event.to.neighbours(), event.to.sides_regular(5.)) {
            let border = map
                .provinces
                .get(&neighbour)
                .is_some_and(|neighbour| neighbour.control != Some(division_country_id));

            if border {
                commands.spawn((
                    Border {
                        between: [event.to, neighbour],
                    },
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

    hovered.0 = map
        .provinces
        .contains_key(&cursor_hpos)
        .then_some(cursor_hpos);
}

pub fn unhighlight(
    mut provs: Query<(&HexagonPos, &mut MeshMaterial2d<ColorMaterial>), With<Mesh2d>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    countries: Query<&Country>,
    map: Res<Map>,
    highlighted: ResMut<Highlighted>,
) {
    let Some(prev_id) = highlighted.0 else { return };

    let (&pos, mut material) = provs.get_mut(prev_id).unwrap();

    // TODO province can be unclaimed
    let prov_owner = map.provinces[&pos].control.unwrap();
    let country = countries.get(prov_owner).unwrap();
    let color = country.color;

    *material = MeshMaterial2d(materials.add(color));
}

pub fn update_highlighted(
    mut provs: Query<(Entity, &HexagonPos, &mut MeshMaterial2d<ColorMaterial>), With<Mesh2d>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut highlighted: ResMut<Highlighted>,
    hovered: Res<HoveredProvince>,
) {
    let Some(hovered_pos) = hovered.0 else {
        highlighted.0 = None;
        return;
    };

    let hovered_province = provs.iter_mut().find(|&(_, &pos, _)| pos == hovered_pos);

    if let Some((id, _, mut material)) = hovered_province {
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

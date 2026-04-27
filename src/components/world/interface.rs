use {
    crate::{components::Map, hexagon_pos::HexagonPos},
    bevy::prelude::*,
};

const COLOR_CHANGE_FACTOR: f32 = 1.1;

#[derive(Resource, Default)]
pub struct HoveredProvince(pub Option<HexagonPos>);

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
        return;
    };

    let cursor_hpos = HexagonPos::from_real_regular(cursor_pos, 5.);

    if map.provinces.contains_key(&cursor_hpos) {
        hovered.0 = Some(cursor_hpos);
    }
}

#[derive(Resource, Default)]
pub struct Highlighted(Option<Entity>);

pub fn unhighlight(
    mut provs: Query<&mut MeshMaterial2d<ColorMaterial>, With<Mesh2d>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    highlighted: ResMut<Highlighted>,
) {
    let Some(prev_id) = highlighted.0 else {
        return;
    };

    let mut material = provs.get_mut(prev_id).unwrap();

    let color = materials.get(material.id()).unwrap().color;
    let color = multiple(color.to_linear(), 1. / COLOR_CHANGE_FACTOR);
    let color = Color::LinearRgba(color);

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

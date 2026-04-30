use {
    crate::{
        country::Country,
        hexagon_pos::HexagonPos,
        plugins::{Map, Owner, Province, SIDE},
    },
    bevy::prelude::*,
};

pub struct ProvinceHoveringPlugin;

impl Plugin for ProvinceHoveringPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveredProvince>()
            .init_resource::<Highlighted>()
            .add_systems(
                Update,
                (
                    update_hovered,
                    (unhighlight, update_highlighted)
                        .chain()
                        .run_if(resource_changed::<HoveredProvince>),
                ),
            );
    }
}

#[derive(Resource, Default)]
pub struct HoveredProvince(pub Option<HexagonPos>);

fn update_hovered(
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

    let cursor_hpos = HexagonPos::from_real_regular(cursor_pos, SIDE);

    if Some(cursor_hpos) == hovered.0 {
        return;
    }

    hovered.0 = map.provs.contains_key(&cursor_hpos).then_some(cursor_hpos);
}

#[derive(Resource, Default)]
pub struct Highlighted(Option<Entity>);

fn unhighlight(
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

fn update_highlighted(
    mut provs: Query<(Entity, &Province, &mut MeshMaterial2d<ColorMaterial>)>,
    owners: Query<&Owner>,
    countries: Query<&Country>,
    mut highlighted: ResMut<Highlighted>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        let owner = owners.get(id).unwrap();
        let country = countries.get(owner.0.unwrap()).unwrap();

        material.0 = materials.add(country.color.lighter(0.05));

        highlighted.0 = Some(id);
    }
}

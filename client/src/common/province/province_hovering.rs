use {bevy::prelude::*, shared::*};

pub struct ProvinceHoveringPlugin;

impl Plugin for ProvinceHoveringPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(highlight).add_observer(unhighlight);
    }
}

fn highlight(
    event: On<Pointer<Over>>,
    mut provs: Query<(&ProvinceOwner, &mut MeshMaterial2d<ColorMaterial>)>,
    countries: Query<&Country>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let prov = event.entity;

    let Ok(query) = provs.get_mut(prov) else {
        return;
    };

    let (&ProvinceOwner(Some(owner)), mut material) = query else {
        return;
    };

    let owner = countries.get(owner).unwrap();

    material.0 = materials.add(owner.color.lighter(0.05));
}

fn unhighlight(
    event: On<Pointer<Out>>,
    mut provs: Query<(&ProvinceOwner, &mut MeshMaterial2d<ColorMaterial>)>,
    countries: Query<&Country>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let prov = event.entity;

    let Ok(query) = provs.get_mut(prov) else {
        return;
    };

    let (&ProvinceOwner(Some(owner)), mut material) = query else {
        return;
    };

    let owner = countries.get(owner).unwrap();

    material.0 = materials.add(owner.color);
}

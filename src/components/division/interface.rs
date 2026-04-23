use std::collections::HashMap;

use bevy::prelude::*;

use crate::{components::Division, hexagon_pos::HexagonPos};

#[derive(Component)]
pub struct SelectedDivision;

#[derive(Component)]
pub struct SelectionMesh;

#[derive(Resource, Default)]
pub struct DivisionsAtProvince(HashMap<HexagonPos, usize>);

pub fn select_division(
    selected: Option<Single<Entity, With<SelectedDivision>>>,
    old_selection: Option<Single<Entity, With<SelectionMesh>>>,
    divisions: Query<(Entity, &Transform), With<Division>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = camera.into_inner();

    let Some(cursor) = window
        .cursor_position()
        .and_then(|c| camera.viewport_to_world_2d(camera_transform, c).ok())
    else {
        return;
    };

    for (id, &transform) in divisions {
        let rect = Rect::from_center_size(transform.translation.xy(), Vec2::new(4., 2.5));

        if !rect.contains(cursor) {
            continue;
        }

        if let Some(sel) = selected
            && let Some(old) = old_selection
        {
            commands
                .entity(sel.into_inner())
                .remove::<SelectedDivision>();

            commands.entity(old.into_inner()).despawn();
        }

        commands.entity(id).insert(SelectedDivision);

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(4., 2.5).to_ring(0.2))),
            MeshMaterial2d(materials.add(Color::linear_rgb(1., 0.8, 0.1))),
            SelectionMesh,
            transform,
        ));

        return;
    }
}

pub fn init_division_mesh(
    divisions: Query<(&Division, Entity), Without<Mesh2d>>,
    mut divisions_at_prov: ResMut<DivisionsAtProvince>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    let mesh = meshes.add(Rectangle::new(4., 2.5));
    let DivisionsAtProvince(divisions_at_prov) = &mut *divisions_at_prov;

    for (division, id) in divisions {
        let divisions_at_the_prov = divisions_at_prov
            .get(&division.pos)
            .copied()
            .unwrap_or_default();

        let shift = Vec2::new(0., 3. * divisions_at_the_prov as f32);

        *divisions_at_prov.entry(division.pos).or_default() += 1;

        let pos = division.pos.real_regular(5.) + Vec2::new(3., 2.) + shift;

        commands.get_entity(id).unwrap().insert((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(Color::linear_rgb(0.2, 0.8, 0.2))),
            Transform::from_xyz(pos.x, pos.y, 0.),
        ));
    }
}

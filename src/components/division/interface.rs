use {
    crate::{
        components::{Division, DivisionMoved},
        hexagon_pos::HexagonPos,
    },
    bevy::prelude::*,
    std::collections::HashMap,
};

#[derive(Component)]
pub struct SelectedDivision;

#[derive(Component)]
pub struct SelectionMesh;

#[derive(Resource, Default)]
pub struct DivisionsAtProvince(HashMap<HexagonPos, usize>);

pub fn init_division_mesh(
    divisions: Query<(&Division, Entity)>,
    mut divisions_at_prov: ResMut<DivisionsAtProvince>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    divisions_at_prov.0 = HashMap::new();

    let mesh = meshes.add(Rectangle::new(4., 2.5));
    let DivisionsAtProvince(divisions_at_prov) = &mut *divisions_at_prov;

    for (division, id) in divisions {
        let divisions_at_the_prov = divisions_at_prov
            .get(&division.pos)
            .copied()
            .unwrap_or_default();

        let shift = Vec2::new(0., 3. * divisions_at_the_prov as f32);

        *divisions_at_prov.entry(division.pos).or_default() += 1;

        let pos = division.pos.real_regular(5.) + shift;

        commands.entity(id).insert((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(Color::linear_rgb(0.2, 0.8, 0.2))),
            Transform::from_xyz(pos.x, pos.y, 0.),
        ));
    }
}

pub fn update_divisions_mesh(
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

pub fn select_division(
    selected: Option<Single<Entity, With<SelectedDivision>>>,
    divisions: Query<(Entity, &GlobalTransform), With<Division>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
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

    let Some((id, _)) = divisions.iter().find(|(_, transform)| {
        let rect = Rect::from_center_size(transform.translation().xy(), Vec2::new(4., 2.5));

        rect.contains(cursor)
    }) else {
        return;
    };

    if let Some(selected) = selected.map(|s| s.into_inner()) {
        commands.entity(selected).remove::<SelectedDivision>();
    }

    commands.entity(id).insert(SelectedDivision);
}

pub fn draw_selection(
    trigger: On<Insert, SelectedDivision>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    commands.entity(trigger.event_target()).with_child((
        Mesh2d(meshes.add(Rectangle::new(4., 2.5).to_ring(0.2))),
        MeshMaterial2d(materials.add(Color::linear_rgb(1., 0.8, 0.1))),
        SelectionMesh,
        Transform::from_xyz(0., 0., 0.1),
    ));
}

pub fn cancel_selection(
    mut commands: Commands,
    selected: Option<Single<Entity, With<SelectedDivision>>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if let Some(selected) = selected
        && input.just_pressed(KeyCode::Escape)
    {
        commands.entity(*selected).remove::<SelectedDivision>();
    }
}

pub fn undraw_selection(
    trigger: On<Remove, SelectedDivision>,
    children: Query<&Children>,
    selection_meshes: Query<(), With<SelectionMesh>>,
    mut commands: Commands,
) {
    let parent = trigger.event_target();

    let Ok(children) = children.get(parent) else {
        return;
    };

    for &child in children {
        if selection_meshes.contains(child) {
            commands.entity(child).despawn();
        }
    }
}

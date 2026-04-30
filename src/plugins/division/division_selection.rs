use {crate::plugins::Division, bevy::prelude::*};

pub struct DivisionSelectionPlugin;

impl Plugin for DivisionSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (select_division, cancel_selection))
            .add_observer(selection_mesh)
            .add_observer(undraw_selection);
    }
}

#[derive(Component)]
pub struct SelectedDivision;

fn select_division(
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

#[derive(Component)]
pub struct SelectionMesh;

fn selection_mesh(
    trigger: On<Insert, SelectedDivision>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    commands.entity(trigger.event_target()).with_child((
        SelectionMesh,
        Transform::from_xyz(0., 0., 0.1),
        Mesh2d(meshes.add(Rectangle::new(4., 2.5).to_ring(0.2))),
        MeshMaterial2d(materials.add(Color::linear_rgb(1., 0.8, 0.1))),
    ));
}

fn cancel_selection(
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

fn undraw_selection(
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

use {
    bevy::{input::keyboard::Key, prelude::*},
    shared::*,
};

pub struct DivisionSelectionPlugin;

impl Plugin for DivisionSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cancel_selection)
            .add_observer(add_selection_mesh)
            .add_observer(remove_selection_mesh)
            .add_observer(select_division);
    }
}

#[derive(Component)]
pub struct SelectedDivision;

fn select_division(
    event: On<Pointer<Click>>,
    selected: Query<Entity, With<SelectedDivision>>,
    divisions: Query<Entity, With<Division>>,
    keys: Res<ButtonInput<Key>>,
    mut commands: Commands,
) {
    let clicked = event.entity;

    if !divisions.contains(clicked) {
        return;
    }

    if !keys.pressed(Key::Shift) {
        for entity in selected {
            commands.entity(entity).remove::<SelectedDivision>();
        }
    }

    commands.entity(clicked).insert(SelectedDivision);
}

fn cancel_selection(
    mut commands: Commands,
    selected: Option<Single<Entity, With<SelectedDivision>>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Some(selected) = selected
        && keys.just_pressed(KeyCode::Escape)
    {
        commands.entity(*selected).remove::<SelectedDivision>();
    }
}

#[derive(Component)]
pub struct SelectionMesh;

fn add_selection_mesh(
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

fn remove_selection_mesh(
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

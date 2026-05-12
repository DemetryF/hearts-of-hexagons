mod division_mesh;
mod division_movement;
mod division_selection;
mod ui;

pub use {division_mesh::*, division_movement::*, division_selection::*, ui::*};

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DivisionMeshPlugin,
            DivisionMovementPlugin,
            DivisionSelectionPlugin,
            UiPlugin,
        ));
    }
}

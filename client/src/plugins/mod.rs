mod controls;
mod division_mesh;
mod division_movement;
mod division_selection;
mod interface;
mod province;

pub use {
    controls::*, division_mesh::*, division_movement::*, division_selection::*, interface::*,
    province::*,
};

use bevy::prelude::*;

pub struct ClientPlugins;

impl Plugin for ClientPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ControlsPlugin,
            DivisionMeshPlugin,
            DivisionSelectionPlugin,
            DivisionMovementPlugin,
            UiPlugin,
            ProvincePlugin,
        ));
    }
}

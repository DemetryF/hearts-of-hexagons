mod connecting_menu;
mod lobby;
mod plugins;

use bevy::{input_focus::InputFocus, prelude::*};
use bevy_replicon::prelude::*;
use bevy_replicon_renet::RepliconRenetPlugins;
use bevy_simple_text_input::TextInputPlugin;
use shared::*;

use crate::{
    connecting_menu::ConnectingMenuPlugin,
    lobby::LobbyPlugin,
    plugins::{
        ControlsPlugin, DivisionMeshPlugin, DivisionMovementPlugin, DivisionSelectionPlugin,
        GameUiPlugin, ProvincePlugin,
    },
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TextInputPlugin,
            RepliconPlugins,
            RepliconRenetPlugins,
            ProtocolPlugin,
            // connecting menu
            ConnectingMenuPlugin,
            // lobby
            ProvincePlugin,
            ControlsPlugin,
            LobbyPlugin,
            // game
            DivisionMeshPlugin,
            DivisionMovementPlugin,
            DivisionSelectionPlugin,
            GameUiPlugin,
        ))
        .init_resource::<InputFocus>()
        .init_resource::<PlayingCountry>()
        .init_state::<AppState>()
        .add_systems(Startup, init_cam)
        .run();
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    None,
    Lobby,
    Game,
}

fn init_cam(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource, Default)]
pub struct PlayingCountry(pub Option<Entity>);

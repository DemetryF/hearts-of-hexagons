mod common;
mod connecting_menu;
mod game;
mod lobby;

use {
    bevy::{input_focus::InputFocus, prelude::*},
    bevy_replicon::prelude::*,
    bevy_replicon_renet::RepliconRenetPlugins,
    bevy_simple_text_input::TextInputPlugin,
    shared::*,
};

use crate::{
    common::CommonPlugins, connecting_menu::ConnectingMenuPlugin, game::GamePlugin,
    lobby::LobbyPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TextInputPlugin,
            RepliconPlugins,
            RepliconRenetPlugins,
            ProtocolPlugin,
            ConnectingMenuPlugin,
            CommonPlugins,
            LobbyPlugin,
            GamePlugin,
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
    ConnectingMenu,
    Lobby,
    Game,
}

fn init_cam(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource, Default)]
pub struct PlayingCountry(pub Option<Entity>);

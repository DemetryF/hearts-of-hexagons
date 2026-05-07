mod plugins;

use bevy::{input_focus::InputFocus, prelude::*};
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    RenetChannelsExt, RenetClient, RepliconRenetPlugins, netcode::*, renet::*,
};
use shared::{Country, CountryAssignment, ProtocolPlugin, Province};
use std::{net::UdpSocket, time::SystemTime};

use crate::plugins::{ClientPlugins, Map};

const PORT: u16 = 5000;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RepliconPlugins,
            ClientPlugins,
            ProtocolPlugin,
            RepliconRenetPlugins,
        ))
        .init_resource::<PlayingCountry>()
        .init_resource::<InputFocus>()
        .add_systems(Startup, (init_client, init_cam))
        .add_systems(Update, send_country_assignment_request)
        .add_systems(PreUpdate, add_province_to_map.after(ClientSystems::Receive))
        .run();
}

fn init_client(mut commands: Commands, channels: Res<RepliconChannels>) {
    let connection_config = ConnectionConfig {
        server_channels_config: channels.server_configs(),
        client_channels_config: channels.client_configs(),
        ..default()
    };

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let auth = ClientAuthentication::Unsecure {
        protocol_id: 1,
        client_id: now.as_millis() as u64,
        server_addr: format!("127.0.0.1:{PORT}").parse().unwrap(),
        user_data: None,
    };

    commands.insert_resource(RenetClient::new(connection_config));
    commands.insert_resource(NetcodeClientTransport::new(now, auth, socket).unwrap());
}

fn init_cam(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource, Default)]
pub struct PlayingCountry(pub Option<Entity>);

fn send_country_assignment_request(
    countries: Query<(Entity, &Country)>,
    mut commands: Commands,
    mut playing_country: ResMut<PlayingCountry>,
) {
    if playing_country.0.is_some() {
        return;
    }

    for (entity, country) in countries {
        if country.name == "Germany" {
            commands.client_trigger(CountryAssignment { entity });
            playing_country.0 = Some(entity);
        }
    }
}

fn add_province_to_map(
    mut map: ResMut<Map>,
    provinces: Query<(Entity, &Province), Added<Province>>,
) {
    for (id, prov) in provinces {
        map.provs.insert(prov.pos, id);
    }
}

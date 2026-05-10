mod map;
mod plugins;

use crate::{map::Map, plugins::GamePlugins};
use bevy::{prelude::*, state::app::StatesPlugin};
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    RenetChannelsExt, RenetServer, RepliconRenetPlugins, netcode::*, renet::ConnectionConfig,
};
use serde::Deserialize;
use shared::*;
use std::{collections::HashMap, fs, net::UdpSocket, time::SystemTime};

const PORT: u16 = 5000;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            StatesPlugin,
            RepliconPlugins,
            RepliconRenetPlugins,
            ProtocolPlugin,
            GamePlugins,
        ))
        .init_resource::<Map>()
        .init_resource::<Players>()
        .add_systems(
            Startup,
            (init_server, init_map_n_countries, spawn_divisions).chain(),
        )
        .add_observer(assign_country)
        .run();
}

fn init_server(mut commands: Commands, channels: Res<RepliconChannels>) {
    let connection_config = ConnectionConfig {
        server_channels_config: channels.server_configs(),
        client_channels_config: channels.client_configs(),
        ..default()
    };

    let socket = UdpSocket::bind(("0.0.0.0", PORT)).unwrap();

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let cfg = ServerConfig {
        current_time: now,
        max_clients: 2,
        protocol_id: 1,
        public_addresses: vec![format!("127.0.0.1:{PORT}").parse().unwrap()],
        authentication: ServerAuthentication::Unsecure,
    };

    commands.insert_resource(RenetServer::new(connection_config));
    commands.insert_resource(NetcodeServerTransport::new(cfg, socket).unwrap());
}

fn spawn_divisions(mut commands: Commands, countries: Query<(&Country, Entity)>) {
    let (_, id) = (countries.into_iter())
        .find(|(country, _)| &country.name == "France")
        .unwrap();

    commands.spawn((
        Division {
            organization: 20.,
            max_organization: 40.,
            hp: 100.,
            max_hp: 120.,
            speed: 10.,
        },
        DivisionPos(HexagonPos { x: 30, y: -44 }),
        CombatStats {
            attack: 9.,
            defend: 9.,
            breakthrough: 9.,
        },
        DivisionOwner(id),
    ));

    let (_, id) = (countries.into_iter())
        .find(|(country, _)| &country.name == "Germany")
        .unwrap();

    commands.spawn((
        Division {
            organization: 40.,
            max_organization: 40.,
            hp: 100.,
            max_hp: 120.,
            speed: 10.,
        },
        DivisionPos(HexagonPos { x: 32, y: -41 }),
        CombatStats {
            attack: 40.,
            defend: 10.,
            breakthrough: 10.,
        },
        DivisionOwner(id),
    ));

    commands.spawn((
        Division {
            organization: 40.,
            max_organization: 40.,
            hp: 100.,
            max_hp: 120.,
            speed: 10.,
        },
        DivisionPos(HexagonPos { x: 32, y: -41 }),
        CombatStats {
            attack: 40.,
            defend: 10.,
            breakthrough: 10.,
        },
        DivisionOwner(id),
    ));

    println!("spawned 3 divisions")
}

fn init_map_n_countries(mut commands: Commands, mut map: ResMut<Map>) {
    let (hexagons, countries) = world_from_json();

    let mut countries_id = HashMap::new();

    for (color, name) in countries.into_iter() {
        let entity = {
            let color = Color::linear_rgb(
                color[0] as f32 / 255.,
                color[1] as f32 / 255.,
                color[2] as f32 / 255.,
            );

            commands
                .spawn(Country {
                    name,
                    color,
                    money: 0,
                })
                .id()
        };

        countries_id.insert(color, entity);
    }

    for (pos, color) in hexagons {
        let control = countries_id[&color];

        let id = commands
            .spawn((Province { pos }, ProvinceOwner(Some(control))))
            .id();

        map.provs.insert(pos, id);
    }
}

fn world_from_json() -> (HashMap<HexagonPos, [u8; 4]>, HashMap<[u8; 4], String>) {
    #[derive(Deserialize)]
    struct Data {
        hexagons: HashMap<String, [u8; 4]>,
        countries: HashMap<String, [u8; 4]>,
    }

    let data = fs::read_to_string("./countries.json").unwrap();
    let data: Data = serde_json::from_str(&data).unwrap();

    let countries: HashMap<[u8; 4], String> = data
        .countries
        .into_iter()
        .map(|(name, color)| (color, name))
        .collect();

    println!("loaded {} provinces", data.hexagons.len());

    let hexagons = data
        .hexagons
        .into_iter()
        .map(|(pos, color)| {
            let (x, y) = pos.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();

            (HexagonPos::new(x, -y), color)
        })
        .collect();

    println!("loaded {} countries", countries.len());

    (hexagons, countries)
}

/// <Client, Country> table
#[derive(Resource, Default)]
pub struct Players(pub HashMap<Entity, Entity>);

fn assign_country(
    req: On<FromClient<CountryAssignmentRequest>>,
    mut players: ResMut<Players>,
    mut commands: Commands,
) {
    let client = req.client_id.entity().unwrap();
    let country = req.message.entity;

    let message = {
        if (players.0.values()).any(|&other_country| other_country == country) {
            CountryAssignmentResponse::CountryIsBusy
        } else {
            players.0.insert(client, country);
            commands.entity(client).insert(Plays(country));

            CountryAssignmentResponse::Success(country)
        }
    };

    commands.server_trigger(ToClients {
        mode: SendMode::Direct(req.client_id),
        message,
    });
}

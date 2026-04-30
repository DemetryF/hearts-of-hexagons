use {
    crate::{
        country::{Country, PlayingCountry},
        hexagon_pos::HexagonPos,
        plugins::*,
    },
    bevy::{input_focus::InputFocus, prelude::*},
    serde::Deserialize,
    std::{collections::HashMap, fs},
};

mod country;
mod hexagon_pos;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            PreStartup,
            ((setup, init_map_n_countries, setup_playing_country).chain(),),
        )
        .add_plugins((
            DivisionPlugin,
            ProvincePlugin,
            ControlsPlugin,
            InterfacePlugin,
            EconomyPlugin,
            TickPlugin,
        ))
        .init_resource::<InputFocus>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_playing_country(mut commands: Commands, countries: Query<(&Country, Entity)>) {
    for (country, id) in countries {
        if &country.name == "Germany" {
            commands.entity(id).insert(PlayingCountry);
            println!("playing country is Germany");

            commands.spawn(Division {
                hp: 100,
                max_hp: 120,
                attack: 10,
                defend: 10,
                speed: 10.,
                pos: HexagonPos { x: 32, y: -41 },
                country: id,
            });

            commands.spawn(Division {
                hp: 100,
                max_hp: 120,
                attack: 10,
                defend: 10,
                speed: 10.,
                pos: HexagonPos { x: 32, y: -41 },
                country: id,
            });

            commands.spawn(Division {
                hp: 100,
                max_hp: 120,
                attack: 10,
                defend: 10,
                speed: 10.,
                pos: HexagonPos { x: 32, y: -41 },
                country: id,
            });

            break;
        }
    }
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
            .spawn((Province { pos }, Owner(Some(control))))
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

    let data = fs::read_to_string("countries.json").unwrap();
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

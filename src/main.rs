use std::{collections::HashMap, fs};

use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    components::{
        Country, Division, DivisionsAtProvince, Highlighted, HoveredProvince, Map, PlayingCountry,
        Province, calculate_path, cancel_selection, draw_selection, end_moving, init_division_mesh,
        process_moving, regenerate_division, select_division, setup_provinces_meshes, start_moving,
        undraw_selection, unhighlight, update_highlighted, update_hovered,
    },
    hexagon_pos::HexagonPos,
    systems::{
        camera_movement, camera_zoom, display_country_info, gain_money, update_country_info,
    },
    tick::{Tick, run_tick, setup_ticks},
};

mod components;
mod hexagon_pos;
mod systems;
mod tick;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                (
                    setup,
                    init_map_n_countries,
                    setup_provinces_meshes,
                    setup_playing_country,
                    display_country_info,
                )
                    .chain(),
                setup_ticks,
            ),
        )
        .add_systems(
            Update,
            (
                run_tick,
                select_division,
                init_division_mesh,
                update_hovered,
                cancel_selection,
                regenerate_division,
                start_moving,
                calculate_path,
                end_moving,
                (unhighlight, update_highlighted)
                    .chain()
                    .run_if(resource_changed::<HoveredProvince>),
            ),
        )
        .add_systems(FixedUpdate, (camera_zoom, camera_movement))
        .add_systems(Tick, (gain_money, update_country_info, process_moving))
        .add_observer(undraw_selection)
        .add_observer(draw_selection)
        .insert_resource(Map::default())
        .insert_resource(DivisionsAtProvince::default())
        .insert_resource(Highlighted::default())
        .insert_resource(HoveredProvince::default())
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
        let Some(&control) = countries_id.get(&color) else {
            panic!("{color:?}")
        };

        map.provinces.insert(
            pos,
            Province {
                control: Some(control),
            },
        );
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

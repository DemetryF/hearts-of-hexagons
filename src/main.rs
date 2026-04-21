use std::{collections::HashMap, f32::consts::PI, fs};

use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    country::Country,
    hexagon_pos::HexagonPos,
    world::{Map, Province},
};

pub mod country;
pub mod hexagon_pos;
pub mod world;

const SIDE: f32 = 5.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (init_map_n_countries, setup_provinces_meshes).chain(),
        )
        .add_systems(FixedUpdate, (camera_zoom, camera_controls))
        .insert_resource(Map::default())
        .run();
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

            commands.spawn(Country { name, color }).id()
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

    println!("map & countries inited");
}

fn setup_provinces_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    countries: Query<&Country>,
    map: Res<Map>,
) {
    println!("setting provinces meshes up");

    commands.spawn(Camera2d);

    let mesh = meshes.add(RegularPolygon::new(SIDE, 6));

    for (&hpos, province) in &map.provinces {
        let pos = hpos.real_regular(SIDE);
        let country = province.control.unwrap();
        let country = countries.get(country).unwrap();
        let color = country.color;

        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(pos.x, pos.y, 0.0).with_rotation(Quat::from_rotation_z(PI / 2.)),
        ));

        for (neighbour, side) in hpos.neighbours().into_iter().zip(hpos.sides_regular(SIDE)) {
            let border = map
                .provinces
                .get(&neighbour)
                .is_some_and(|neighbour| neighbour.control != province.control);

            if border {
                commands.spawn((
                    Mesh2d(meshes.add(Segment2d::new(side.0, side.1))),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::from_xyz(0., 0., 0.0),
                ));
            }
        }
    }
}

fn camera_controls(
    camera_query: Single<&mut Transform, With<Camera2d>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    let mut transform = camera_query.into_inner();

    let fspeed = 600.0 * time.delta_secs();

    // Camera movement controls
    if input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += fspeed;
    }
    if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= fspeed;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= fspeed;
    }
    if input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += fspeed;
    }
}

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

fn camera_zoom(
    mut camera: Query<(&Camera, &mut Transform, &GlobalTransform, &mut Projection)>,
    window: Single<&Window>,
    mut wheel: MessageReader<MouseWheel>,
) {
    if wheel.read().len() == 0 {
        return;
    }

    let scale = {
        wheel.read().fold(1., |scale, ev| {
            let units = match ev.unit {
                MouseScrollUnit::Line => ev.y,
                MouseScrollUnit::Pixel => ev.y / 20.0,
            };

            scale * ops::powf(0.85, units)
        })
    };

    let Ok((camera, mut transform, global, mut projection)) = camera.single_mut() else {
        return;
    };

    let Projection::Orthographic(proj2d) = &mut *projection else {
        return;
    };

    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let Ok(world_before) = camera.viewport_to_world_2d(global, cursor) else {
        return;
    };

    proj2d.scale *= scale;

    let cam_xy = transform.translation.truncate();
    let new_cam = world_before * (1.0 - scale) + cam_xy * scale;
    transform.translation.x = new_cam.x;
    transform.translation.y = new_cam.y;
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

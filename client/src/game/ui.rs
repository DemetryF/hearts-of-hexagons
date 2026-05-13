use {
    crate::{AppState, PlayingCountry},
    bevy::prelude::*,
    shared::*,
};

const BACKGROUND_COLOR: Color = Color::linear_rgba(0.3, 0.3, 0.3, 0.4);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        (app)
            .add_systems(
                OnEnter(AppState::Game),
                (init_country_info, init_hovered_prov_info),
            )
            .add_systems(Update, update_country_info)
            .add_observer(show_hovered_prov_info)
            .add_observer(hide_hovered_prov_info);
    }
}

#[derive(Component)]
pub struct UiMoneyLabel;

#[derive(Component)]
pub struct UiBuyDivisionButton;

fn init_country_info(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(20),
            height: percent(20),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::all(percent(1)),
            border_radius: BorderRadius::all(percent(5)),
            ..Default::default()
        },
        BackgroundColor(BACKGROUND_COLOR),
        children![
            (
                UiMoneyLabel,
                Text::default(),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    margin: UiRect::bottom(px(10)),
                    ..Default::default()
                },
            ),
            (
                UiBuyDivisionButton,
                Button,
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    border_radius: BorderRadius::all(px(20)),
                    padding: UiRect::horizontal(px(10)),
                    ..Default::default()
                },
                children![(
                    Text::new("Buy division"),
                    TextLayout::new_with_justify(Justify::Center),
                )],
            )
        ],
    ));
}

fn update_country_info(
    money_label: Option<Single<&mut Text, With<UiMoneyLabel>>>,
    playing_country: Res<PlayingCountry>,
    countries: Query<&Country>,
) {
    let Some(country) = playing_country.0.map(|id| countries.get(id).unwrap()) else {
        return;
    };

    let mut money_label = money_label.unwrap();

    money_label.0 = format!("{}\nmoney: {}", country.name, country.money);
}

#[derive(Component)]
pub struct UiProvInfoNode;

#[derive(Component)]
pub struct UiProvCoords;

fn init_hovered_prov_info(mut commands: Commands) {
    commands.spawn((
        UiProvInfoNode,
        Node {
            position_type: PositionType::Absolute,
            left: px(0),
            bottom: px(0),

            width: percent(20),
            height: percent(20),

            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,

            margin: UiRect::all(percent(1)),
            border_radius: BorderRadius::all(percent(5)),

            ..Default::default()
        },
        Visibility::Hidden,
        BackgroundColor(BACKGROUND_COLOR),
        children![
            Text::new("Province"),
            (
                Text::new(""),
                UiProvCoords,
                TextLayout::new_with_justify(Justify::Center)
            ),
        ],
    ));
}

fn show_hovered_prov_info(
    event: On<Pointer<Over>>,
    mut coords_label: Single<&mut Text, With<UiProvCoords>>,
    vis: Single<&mut Visibility, With<UiProvInfoNode>>,
    provs: Query<(&Province, &ProvinceOwner)>,
    countries: Query<&Country>,
) {
    let prov = event.entity;

    let Ok((prov, owner)) = provs.get(prov) else {
        return;
    };

    *vis.into_inner() = Visibility::Visible;

    let country = countries.get(owner.0.unwrap()).unwrap();

    coords_label.0 = format!("({}, {})\nowner: {}", prov.pos.x, prov.pos.y, country.name);
}

fn hide_hovered_prov_info(
    event: On<Pointer<Out>>,
    vis: Single<&mut Visibility, With<UiProvInfoNode>>,
    provs: Query<(), With<Province>>,
) {
    if provs.get(event.entity).is_ok() {
        *vis.into_inner() = Visibility::Hidden;
    }
}

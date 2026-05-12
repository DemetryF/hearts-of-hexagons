use {
    crate::{
        AppState, PlayingCountry,
        common::{HoveredProvince, Map},
    },
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
                (display_country_info, init_hovered_prov_info),
            )
            .add_systems(
                Update,
                (update_country_info, update_hovered_prov_info).run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Component)]
pub struct UiMoneyLabel;

#[derive(Component)]
pub struct UiBuyDivisionButton;

#[derive(Component)]
pub struct UiProvInfoNode;

#[derive(Component)]
pub struct UiProvCoords;

fn display_country_info(mut commands: Commands) {
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

fn update_hovered_prov_info(
    mut coords_label: Single<&mut Text, With<UiProvCoords>>,
    vis: Single<&mut Visibility, With<UiProvInfoNode>>,
    owners: Query<&ProvinceOwner>,
    countries: Query<&Country>,
    map: Res<Map>,
    hovered: Res<HoveredProvince>,
) {
    if let Some(hovered) = hovered.0 {
        *vis.into_inner() = Visibility::Visible;

        let prov = map.provs[&hovered];
        let owner = owners.get(prov).unwrap();
        let country = countries.get(owner.0.unwrap()).unwrap();

        coords_label.0 = format!("({}, {})\nowner: {}", hovered.x, hovered.y, country.name);
    } else {
        *vis.into_inner() = Visibility::Hidden;
    };
}

use {
    crate::components::{Country, HoveredProvince, PlayingCountry},
    bevy::{input_focus::InputFocus, prelude::*},
};

const BACKGROUND_COLOR: Color = Color::linear_rgba(0.3, 0.3, 0.3, 0.4);

const BUTTON_NORMAL_COLOR: Color = Color::linear_rgb(0.4, 0.4, 0.4);
const BUTTON_HOVERED_COLOR: Color = Color::linear_rgb(0.43, 0.43, 0.43);
const BUTTON_PRESSED_COLOR: Color = Color::linear_rgb(0.46, 0.46, 0.46);

#[derive(Component)]
pub struct UiMoneyLabel;

#[derive(Component)]
pub struct UiBuyDivisionButton;

#[derive(Component)]
pub struct UiProvInfo;

#[derive(Component)]
pub struct UiProvCoords;

pub fn display_country_info(
    country: Option<Single<&Country, With<PlayingCountry>>>,
    mut commands: Commands,
) {
    let Some(country) = country else {
        return;
    };

    println!("display country info");

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
                Text::new(format!("{}\nmoney: {}\n", country.name, country.money)),
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
                BackgroundColor(BUTTON_NORMAL_COLOR),
                children![(
                    Text::new("Buy division"),
                    TextLayout::new_with_justify(Justify::Center),
                )],
            )
        ],
    ));
}

pub fn buy_division_button(
    mut input_focus: ResMut<InputFocus>,
    query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut Button),
        (Changed<Interaction>, With<UiBuyDivisionButton>),
    >,
) {
    for (id, &interaction, mut color, mut button) in query {
        match interaction {
            Interaction::Pressed => {
                input_focus.set(id);
                color.0 = BUTTON_PRESSED_COLOR;
                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(id);
                color.0 = BUTTON_HOVERED_COLOR;
                button.set_changed();
            }
            Interaction::None => {
                input_focus.set(id);
                color.0 = BUTTON_NORMAL_COLOR;
            }
        }
    }
}

pub fn update_country_info(
    country: Option<Single<&Country, With<PlayingCountry>>>,
    money_label: Option<Single<&mut Text, With<UiMoneyLabel>>>,
) {
    let Some(country) = country else {
        return;
    };

    let mut money_label = money_label.unwrap();

    money_label.0 = format!("{}\nmoney: {}", country.name, country.money);
}

pub fn init_hovered_prov_info(mut commands: Commands) {
    commands.spawn((
        UiProvInfo,
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
        children![Text::new("Province"), (Text::new(""), UiProvCoords)],
    ));
}

pub fn update_hovered_prov_info(
    vis: Single<&mut Visibility, With<UiProvInfo>>,
    mut label: Single<&mut Text, With<UiProvCoords>>,
    hovered: Res<HoveredProvince>,
) {
    if let Some(hovered) = hovered.0 {
        *vis.into_inner() = Visibility::Visible;
        label.0 = format!("({}, {})", hovered.x, hovered.y);
    } else {
        *vis.into_inner() = Visibility::Hidden;
    };
}

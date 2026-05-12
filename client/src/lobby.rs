use {
    crate::{AppState, PlayingCountry},
    bevy::prelude::*,
    bevy_replicon::prelude::*,
    shared::*,
};

const BACKGROUND_COLOR: Color = Color::linear_rgba(0.3, 0.3, 0.3, 0.4);

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ClientState::Connected), (lobby_state, spawn_ui))
            .add_systems(Update, (spawn_country_button, country_assignment_request))
            .add_systems(OnEnter(AppState::Game), despawn_ui)
            .add_observer(assign_country);
    }
}

#[derive(Component)]
pub struct UiCountryButtons;

fn lobby_state(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::Lobby);
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        UiCountryButtons,
        Node {
            position_type: PositionType::Absolute,
            right: px(0),
            top: px(0),
            margin: UiRect::all(percent(1)),
            padding: UiRect::horizontal(px(10)),
            border_radius: BorderRadius::all(percent(5)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,
            justify_content: JustifyContent::Start,
            ..Default::default()
        },
        BackgroundColor(BACKGROUND_COLOR),
    ));
}

#[derive(Component)]
pub struct UiCountryButton(Entity);

fn spawn_country_button(
    ui: Single<Entity, With<UiCountryButtons>>,
    countries: Query<(Entity, &Country), Added<Country>>,
    mut commands: Commands,
) {
    for (entity, country) in countries {
        commands.entity(*ui).with_child((
            UiCountryButton(entity),
            Button,
            Node {
                width: percent(100),
                height: percent(100),
                border_radius: BorderRadius::all(percent(5)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                padding: UiRect::horizontal(px(10)),
                ..Default::default()
            },
            children![(Text::new(&country.name))],
        ));
    }
}

fn country_assignment_request(
    button: Option<Single<(&Interaction, &UiCountryButton), Changed<Interaction>>>,
    mut commands: Commands,
) {
    let Some(&(&interaction, &UiCountryButton(country))) = button.as_deref() else {
        return;
    };

    if interaction == Interaction::Pressed {
        commands.client_trigger(CountryAssignmentRequest { entity: country });
    }
}

fn assign_country(
    response: On<CountryAssignmentResponse>,
    mut playing_country: ResMut<PlayingCountry>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    match response.event() {
        &CountryAssignmentResponse::Success(entity) => {
            playing_country.0 = Some(entity);
            app_state.set(AppState::Game);
        }
        CountryAssignmentResponse::CountryIsBusy => println!("country is busy"),
    }
}

fn despawn_ui(ui: Single<Entity, With<UiCountryButtons>>, mut commands: Commands) {
    let ui = *ui;

    commands.entity(ui).despawn();
}

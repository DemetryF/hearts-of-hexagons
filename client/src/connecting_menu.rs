use {
    bevy::prelude::*,
    bevy_replicon::shared::backend::{ClientState, channels::RepliconChannels},
    bevy_replicon_renet::{
        RenetChannelsExt, RenetClient,
        netcode::{ClientAuthentication, NetcodeClientTransport},
        renet::ConnectionConfig,
    },
    bevy_simple_text_input::{TextInput, TextInputValue},
    std::{net::UdpSocket, time::SystemTime},
};

const BACKGROUND_COLOR: Color = Color::linear_rgba(0.3, 0.3, 0.3, 0.4);

pub struct ConnectingMenuPlugin;

impl Plugin for ConnectingMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_ui)
            .add_systems(Update, connect)
            .add_systems(OnEnter(ClientState::Connected), despawn_connect_menu);
    }
}

#[derive(Component)]
struct Ui;

#[derive(Component)]
struct UiAddress;

#[derive(Component)]
struct UiConnect;

fn init_ui(mut commands: Commands) {
    commands.spawn((
        Ui,
        Node {
            width: percent(100),
            height: percent(100),

            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,

            ..Default::default()
        },
        BackgroundColor(BACKGROUND_COLOR),
        children![
            (Text::new("server address:")),
            (
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(px(2)),
                    padding: UiRect::all(px(5)),
                    margin: UiRect::all(px(10)),
                    width: px(400),
                    ..Default::default()
                },
                BorderColor::all(Color::WHITE),
                children![(UiAddress, TextInput)]
            ),
            (
                UiConnect,
                Button,
                Node {
                    border_radius: BorderRadius::all(px(20)),
                    padding: UiRect::horizontal(px(10)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Default::default()
                },
                children![Text::new("connect")]
            )
        ],
    ));
}

fn connect(
    interaction: Single<&Interaction, (Changed<Interaction>, With<UiConnect>)>,
    address: Single<&TextInputValue, With<UiAddress>>,
    channels: Res<RepliconChannels>,
    mut commands: Commands,
) {
    let &interaction = interaction.into_inner();
    let address = &address.into_inner().0;

    if interaction == Interaction::Pressed {
        {
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
                server_addr: address.parse().unwrap(),
                user_data: None,
            };

            commands.insert_resource(RenetClient::new(connection_config));
            commands.insert_resource(NetcodeClientTransport::new(now, auth, socket).unwrap());

            println!("connected");
        }
    }
}

fn despawn_connect_menu(ui: Single<Entity, With<Ui>>, mut commands: Commands) {
    commands.entity(ui.into_inner()).despawn();

    println!("despawned ui");
}

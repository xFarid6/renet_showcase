use std::net::SocketAddr;

use bevy::prelude::*;
use bevy::window::PresentMode;
use local_ip_address::local_ip;
use renet_showcase::{RenetClientPlugin, RenetClient, ClientAuthentication, system_adapter::unwrap};
use tokio::net::UdpSocket;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "I am a window!".to_string(),
                width: 500.,
                height: 300.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(RenetClientPlugin{ clear_events: true })
        .insert_resource(create_renet_client())
        .run();
}

fn create_renet_client() -> RenetClient {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let clinet_id: u64 = current_time.as_millis() as u64;

    let connection_config = RenetConnectionConfig::default();

    // TODO: Prompt user for server address
    let server_address = SocketAddr::new(local_ip().unwrap(), 42069);

    let authentication = ClientAuthentication::Unsecure {
        client_id;
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    RenetClient::new(
        current_time,
        socket,
        clinet_id
        connection_config,
        authentication,
    )
    .unwrap()
}
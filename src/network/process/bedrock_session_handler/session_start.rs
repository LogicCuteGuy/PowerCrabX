use bedrockrs::proto::connection::Connection;
use bedrockrs::proto::v662::enums::{ConnectionFailReason, PacketCompressionAlgorithm};
use bedrockrs::proto::v662::packets::{NetworkSettingsPacket, RequestNetworkSettingsPacket};
use bedrockrs::proto::v729::packets::play_status::PlayStatusPacket;
use bedrockrs::proto::v729::types::play_status::PlayStatusType;
use bedrockrs::proto::v748::packets::{DisconnectPacket, DisconnectPacketMessage};
use bedrockrs::proto::v785::gamepackets::GamePackets;
use bedrockrs::proto::v785::helper::ProtoHelperV785;
use crate::network::protocol_info::CURRENT_PROTOCOL;

pub async fn handle(mut conn: &mut Connection<ProtoHelperV785>, data: &RequestNetworkSettingsPacket) {
    let protocol = data.client_network_version;
    if (protocol != CURRENT_PROTOCOL) {
        let message = if protocol < CURRENT_PROTOCOL {"disconnectionScreen.outdatedClient"} else {"disconnectionScreen.outdatedServer"};
        let status = if protocol < CURRENT_PROTOCOL {PlayStatusType::FailedClientOld} else {PlayStatusType::FailedServerOld};
        conn.send(&[
            GamePackets::PlayStatus(PlayStatusPacket {
                status
            }),
            GamePackets::Disconnect(DisconnectPacket {
                reason: ConnectionFailReason::Unknown,
                messages: Some(DisconnectPacketMessage { message: String::from(message), filtered_message: String::new() })
            })
        ])
            .await
            .unwrap();
        conn.close().await;
        return;
    }

    conn.send(&[GamePackets::NetworkSettings(NetworkSettingsPacket {
        compression_threshold: 1,
        compression_algorithm: PacketCompressionAlgorithm::ZLib,
        client_throttle_enabled: false,
        client_throttle_threshold: 0,
        client_throttle_scalar: 0.0,
    })])
        .await
        .unwrap();
    println!("NetworkSettings");


}
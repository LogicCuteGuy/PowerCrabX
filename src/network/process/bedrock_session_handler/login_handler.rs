use bedrockrs::proto::connection::Connection;
use bedrockrs::proto::error::ConnectionError;
use bedrockrs::proto::v662::enums::{ConnectionFailReason, PacketCompressionAlgorithm};
use bedrockrs::proto::v662::packets::{NetworkSettingsPacket, RequestNetworkSettingsPacket};
use bedrockrs::proto::v729::packets::login::LoginPacket;
use bedrockrs::proto::v729::packets::play_status::PlayStatusPacket;
use bedrockrs::proto::v729::types::play_status::PlayStatusType;
use bedrockrs::proto::v748::packets::{DisconnectPacket, DisconnectPacketMessage};
use bedrockrs::proto::v785::gamepackets::GamePackets;
use bedrockrs::proto::v785::helper::ProtoHelperV785;
use crate::network::protocol_info::CURRENT_PROTOCOL;

pub async fn handle(mut conn: &mut Connection<ProtoHelperV785>, packet_data: &LoginPacket) -> Result<(), ConnectionError> {
    let chain_data = &packet_data.connection_request;
    //mock
    let must_xbox = false;
    println!("test");
    if (!chain_data.is_xbox_auth() && false) {
        conn.send(&[
            GamePackets::PlayStatus(PlayStatusPacket {
                status: PlayStatusType::LoginSuccess
            }),
            GamePackets::Disconnect(DisconnectPacket {
                reason: ConnectionFailReason::Unknown,
                messages: Some(DisconnectPacketMessage {
                    message: String::from("disconnectionScreen.notAuthenticated"),
                    filtered_message: String::new()
                })
            })
        ])
            .await?;
        conn.close().await;
        return Ok(());
    }


    Ok(())
}
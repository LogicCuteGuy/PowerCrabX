use bedrockrs::addon::Addon;
use bedrockrs::addon::resource::ResourcePack;
use bedrockrs::proto::compression::Compression;
use bedrockrs::proto::v662::enums::{ConnectionFailReason, PacketCompressionAlgorithm};
use bedrockrs::proto::v662::packets::{ClientToServerHandshakePacket, NetworkSettingsPacket, RequestNetworkSettingsPacket};
use bedrockrs::proto::v729::packets::play_status::PlayStatusPacket;
use bedrockrs::proto::v729::types::play_status::PlayStatusType;
use bedrockrs::proto::v748::packets::{DisconnectPacket, DisconnectPacketMessage};
use bedrockrs::proto::v766::packets::{ResourcePackEntry, ResourcePacksInfoPacket};
use bedrockrs::proto::v785::gamepackets::GamePackets;
use serde::de::Unexpected::Str;
use uuid::Uuid;
use crate::network::connection::bedrock_session::{BedrockSession, SessionState};
use crate::network::protocol_info::CURRENT_PROTOCOL;

pub async fn handle(mut session: &mut BedrockSession, packet_data: &ClientToServerHandshakePacket) {
    println!("handShake");
    session.change_state(SessionState::ResourcePack);
    let res = ResourcePack::import("./res/resource_pack.zip").unwrap();
    println!("res {:?}", res.get_pack_size());
    session.send(&[
        GamePackets::ResourcePacksInfo(ResourcePacksInfoPacket {
            resource_pack_required: false,
            has_addon_packs: false,
            has_scripts: false,
            world_template_uuid: Uuid::new_v4(),
            resource_packs: vec![
                ResourcePackEntry {
                    id: res.manifest.header.uuid,
                    version: res.manifest.header.version.to_string(),
                    size: res.get_pack_size(),
                    content_key: String::new(),
                    sub_pack_name: String::new(),
                    content_identity: String::new(),
                    has_scripts: false,
                    is_addon_pack: false,
                    is_ray_tracing_capable: false,
                    cdn_url: String::new(),
                }
            ],
            world_template_version: String::new(),
        })
    ]).await;
}
use bedrockrs::addon::Addon;
use bedrockrs::addon::resource::ResourcePack;
use bedrockrs::proto::compression::Compression;
use bedrockrs::proto::v662::enums::{ConnectionFailReason, PacketCompressionAlgorithm};
use bedrockrs::proto::v662::packets::{NetworkSettingsPacket, RequestNetworkSettingsPacket, ResourcePackChunkDataPacket, ResourcePackChunkRequestPacket};
use bedrockrs::proto::v729::packets::play_status::PlayStatusPacket;
use bedrockrs::proto::v729::types::play_status::PlayStatusType;
use bedrockrs::proto::v748::packets::{DisconnectPacket, DisconnectPacketMessage};
use bedrockrs::proto::v785::gamepackets::GamePackets;
use crate::network::connection::bedrock_session::{BedrockSession, SessionState};
use crate::network::protocol_info::CURRENT_PROTOCOL;

pub async fn handle(mut session: &mut BedrockSession, packet_data: &ResourcePackChunkRequestPacket) {
    let mut res = ResourcePack::import("./res/resource_pack.zip").unwrap();
    let max_chunk_size = 1024 * 100;
    let remaining = res.get_pack_size() - (max_chunk_size * packet_data.chunk) as u64;
    println!("remaining {}", remaining);
    let len = remaining.min(max_chunk_size as u64);
    println!("len {}", len);
    let chunkdata =  res.get_pack_chunk((max_chunk_size * packet_data.chunk) as u64, len as usize).unwrap();
    println!("dawada {:?}", chunkdata.len());
    session.send(&[
        GamePackets::ResourcePackChunkData(ResourcePackChunkDataPacket {
            resource_name: packet_data.resource_name.clone(),
            chunk_id: packet_data.chunk,
            byte_offset: (len * packet_data.chunk as u64),
            chunk_data: chunkdata,
        })
    ]).await;
}
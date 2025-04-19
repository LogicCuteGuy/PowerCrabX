use bedrockrs::addon::Addon;
use bedrockrs::addon::resource::ResourcePack;
use crate::network::connection::bedrock_session::{BedrockSession, SessionState};
use bedrockrs::proto::v662::enums::{ConnectionFailReason, PackType, ResourcePackResponse};
use bedrockrs::proto::v662::packets::{ClientToServerHandshakePacket, PlayStatusPacket, ResourcePackClientResponsePacket, ResourcePackDataInfoPacket};
use bedrockrs::proto::v729::types::base_game_version::BaseGameVersion;
use bedrockrs::proto::v729::types::experiments::{Experiment, Experiments};
use bedrockrs::proto::v748::packets::{DisconnectPacket, DisconnectPacketMessage, PackEntry, ResourcePackStackPacket};
use bedrockrs::proto::v766::packets::ResourcePacksInfoPacket;
use bedrockrs::proto::v785::gamepackets::GamePackets;
use log::{info, log};
use uuid::Uuid;
use vek::num_traits::real::Real;

pub async fn handle(
    mut session: &mut BedrockSession,
    packet_data: &ResourcePackClientResponsePacket,
) {
    println!("ResourcePackClientResponsePacket");
    match packet_data.response {
        ResourcePackResponse::Refused => {
            println!("Refused");
            session
                .send(&[GamePackets::Disconnect(DisconnectPacket {
                    reason: ConnectionFailReason::Unknown,
                    messages: Some(DisconnectPacketMessage {
                        message: String::from("disconnectionScreen.noReason"),
                        filtered_message: String::new(),
                    }),
                })])
                .await;
            session.close().await;
        }
        ResourcePackResponse::SendPacks => {
            for uuid in &packet_data.downloading_packs {
                let res = ResourcePack::import("./res/resource_pack.zip").unwrap();
                // println!("res {:?}", res.sha256);
                println!("Downloading pack: {}", uuid);
                // if !res.manifest.header.uuid.to_string().eq(uuid) {
                //     session.send(&[
                //         GamePackets::Disconnect(DisconnectPacket {
                //             reason: ConnectionFailReason::Unknown,
                //             messages: Some(DisconnectPacketMessage {
                //                 message: String::from("disconnectionScreen.resourcePack"),
                //                 filtered_message: String::new(),
                //             }),
                //         }),
                //     ]).await;
                //     return;
                // }

                let num_chunks = ((res.get_pack_size() as f64) / (1024.0 * 100.0)).ceil() as u32;

                let data_info_packet = ResourcePackDataInfoPacket {
                    resource_name: uuid.clone(),
                    chunk_size: 1024 * 100,
                    chunk_amount: num_chunks,
                    file_size: res.get_pack_size(),
                    file_hash: res.sha256,
                    is_premium: false,
                    pack_type: PackType::Resources,
                };
                session.send(&[GamePackets::ResourcePackDataInfo(data_info_packet.clone())]).await;
            }
        }
        ResourcePackResponse::HaveAllPacks => {
            // let resource_pack = ResourcePack {
            //     manifest: AddonManifest {},
            //     languages: Languages {},
            // };
            session
                .send(&[GamePackets::ResourcePackStack(ResourcePackStackPacket {
                    texture_pack_required: false,
                    addon_list: vec![],
                    texture_pack_list: vec![],
                    base_game_version: BaseGameVersion(String::from("1.0")),
                    experiments: Experiments {
                        experiments: vec![
                            Experiment {
                                name: String::from("data_driven_items"),
                                enabled: true,
                            },
                            Experiment {
                                name: String::from("data_driven_biomes"),
                                enabled: true,
                            },
                            Experiment {
                                name: String::from("upcoming_creator_features"),
                                enabled: true,
                            },
                            Experiment {
                                name: String::from("gametest"),
                                enabled: true,
                            },
                            Experiment {
                                name: String::from("experimental_molang_features"),
                                enabled: true,
                            },
                            Experiment {
                                name: String::from("cameras"),
                                enabled: true,
                            },
                        ],
                        ever_toggled: true,
                    },
                    include_editor_packs: false,
                })])
                .await;
        }
        ResourcePackResponse::Completed => {
            info!("ResourcePackClientResponsePacket STATUS_COMPLETED");
            session.change_state(SessionState::PreSpawn);
        }
    }
}

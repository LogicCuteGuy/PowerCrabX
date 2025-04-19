use crate::network::process::bedrock_session_handler::{login_handler, resource_pack_chunk_request_handler, resource_pack_handler};
use crate::network::process::bedrock_session_handler::session_start;
use crate::network::process::bedrock_session_handler::client_to_server_handler;
use bedrockrs::proto::compression::Compression;
use bedrockrs::proto::connection::Connection;
use bedrockrs::proto::encryption::Encryption;
use bedrockrs::proto::error::ConnectionError::TransportError;
use bedrockrs::proto::error::{ConnectionError, TransportLayerError};
use bedrockrs::proto::v662::enums::{
    Difficulty, Dimension, EditorWorldType,
    EducationEditionOffer, GamePublishSetting, GameType, Gamemode, GeneratorType
    , PlayerPermissionLevel, ServerAuthMovementMode,
};
use bedrockrs::proto::v662::packets::LevelChunkPacket;
use bedrockrs::proto::v662::types::{
    ActorRuntimeID, ActorUniqueID, GameRulesChangedPacketData, NetworkBlockPosition
    , SyncedPlayerMovementSettings,
};
use bedrockrs::proto::v729::packets::play_status::PlayStatusPacket;
use bedrockrs::proto::v729::types::base_game_version::BaseGameVersion;
use bedrockrs::proto::v729::types::chat_restriction_level::ChatRestrictionLevel;
use bedrockrs::proto::v729::types::chunk_pos::ChunkPos;
use bedrockrs::proto::v729::types::edu_shared_uri_resource::EduSharedResourceUri;
use bedrockrs::proto::v729::types::experiments::Experiments;
use bedrockrs::proto::v729::types::network_permissions::NetworkPermissions;
use bedrockrs::proto::v729::types::play_status::PlayStatusType;
use bedrockrs::proto::v729::types::spawn_biome_type::SpawnBiomeType;
use bedrockrs::proto::v729::types::spawn_settings::SpawnSettings;
use bedrockrs::proto::v748::packets::{
    AttributeData, ResourcePackStackPacket,
    UpdateAttributesPacket,
};
use bedrockrs::proto::v748::types::LevelSettings;
use bedrockrs::proto::v766::enums::PlayerListPacketType;
use bedrockrs::proto::v766::packets::{PlayerListPacket, ResourcePacksInfoPacket};
use bedrockrs::proto::v776::packets::{ItemRegistryPacket, StartGamePacket};
use bedrockrs::proto::v785::gamepackets::GamePackets;
use bedrockrs::proto::v785::helper::ProtoHelperV785;
use bedrockrs::proto::ProtoHelper;
use std::collections::HashMap;
use tokio::time::Instant;
use uuid::Uuid;
use vek::{Vec2, Vec3};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SessionState {
    Start,
    Login,
    Encryption,
    ResourcePack,
    PreSpawn,
    InGame,
    Death
}

pub struct BedrockSession {
    connection: Connection<ProtoHelperV785>,
    state: SessionState
}

impl BedrockSession {
    pub fn new(connection: Connection<ProtoHelperV785>) -> BedrockSession {
        BedrockSession { connection, state: SessionState::Start }
    }

    pub fn change_state(&mut self, new_state: SessionState) {
        println!("Transitioning from {:?} to {:?}", self.state, new_state);
        self.state = new_state;
    }

    pub async fn start(&mut self) {
        loop {
            let res = self.connection.recv().await;

            if let Ok(packet) = res {
                for (packet) in packet.iter() {
                    match (self.state, packet) {
                        (SessionState::Start, GamePackets::RequestNetworkSettings(packet_data)) => {
                            session_start::handle(self, packet_data).await;
                        }
                        (SessionState::Login, GamePackets::Login(packet_data)) => {
                            login_handler::handle(self, packet_data).await;
                        }
                        (SessionState::InGame, GamePackets::PlayerAuthInput(data)) => {
                            // println!("PlayerAuthInput: {:?}", data);
                        }
                        (SessionState::Encryption, GamePackets::ClientToServerHandshake(packet_data)) => {
                            println!("ClientToServerHandshake");
                            client_to_server_handler::handle(self, packet_data).await;
                        }
                        (SessionState::ResourcePack, GamePackets::ResourcePackClientResponse(packet_data)) => {
                            println!("packet_data {:?}", packet_data);
                            resource_pack_handler::handle(self, packet_data).await;
                        }
                        (SessionState::ResourcePack, GamePackets::ResourcePackChunkRequest(packet_data)) => {
                            resource_pack_chunk_request_handler::handle(self, packet_data).await;
                        }
                        (_, packet) => {
                            println!("packet {:?} in state {:?}", packet, self.state);
                        }
                    }
                }
            } else {
                println!("Connection closed: {:?}", res);
                break
            }
        }
    }

    pub async fn set_compression(
        &mut self,
        compression: Compression,
    ) {

        self.connection.compression = Some(compression)
    }

    pub async fn set_encryption(
        &mut self,
        encryption: Encryption,
    ) {

        self.connection.encryption = Some(encryption)
    }

    pub async fn close(&mut self) {
        self.connection.close().await;
    }

    pub async fn send(
        &mut self,
        gamepackets: &[<ProtoHelperV785 as ProtoHelper>::GamePacketType], // Use ProtoHelperV785 instead of T
    ) {
        self.connection.send(gamepackets).await.expect("Err send gamepackets");
    }

    pub async fn handle_login(&mut self) {
        let time_start = Instant::now();

        // NetworkSettingsRequest
        // self.connection.recv().await.unwrap();
        // println!("NetworkSettingsRequest");

        let compression = Compression::Zlib {
            threshold: 1,
            compression_level: 7,
        };

        // NetworkSettings
        // self.connection.send(&[GamePackets::NetworkSettings(NetworkSettingsPacket {
        //     compression_threshold: 1,
        //     compression_algorithm: PacketCompressionAlgorithm::ZLib,
        //     client_throttle_enabled: false,
        //     client_throttle_threshold: 0,
        //     client_throttle_scalar: 0.0,
        // })])
        //     .await
        //     .unwrap();
        // println!("NetworkSettings");

        self.connection.compression = Some(compression);

        let login_data = self.connection.recv().await.unwrap();

        // Login
        println!("Login data {:?}", login_data);

        self.connection
            .send(&[
                GamePackets::PlayStatus(PlayStatusPacket {
                    status: PlayStatusType::LoginSuccess,
                }),
                GamePackets::ResourcePacksInfo(ResourcePacksInfoPacket {
                    resource_pack_required: false,
                    has_addon_packs: false,
                    has_scripts: false,
                    world_template_uuid: Default::default(),
                    resource_packs: vec![],
                    world_template_version: "".to_string(),
                }),
                GamePackets::ResourcePackStack(ResourcePackStackPacket {
                    texture_pack_required: false,
                    addon_list: vec![],
                    base_game_version: BaseGameVersion(String::from("1.0")),
                    experiments: Experiments {
                        experiments: vec![],
                        ever_toggled: false,
                    },
                    texture_pack_list: vec![],
                    include_editor_packs: false,
                }),
            ])
            .await
            .unwrap();
        println!("PlayStatus (LoginSuccess)");
        println!("ResourcePacksInfo");
        println!("ResourcePackStack");

        // conn.send(&[
        //     GamePackets::Disconnect(DisconnectPacket {
        //         reason: ConnectionFailReason::Unknown,
        //         messages: Some(DisconnectPacketMessage{
        //             message: String::from("Test Disconnected"),
        //             filtered_message: String::from("")
        //         }),
        //     })
        // ])
        // .await
        // .unwrap();

        let packet1 = StartGamePacket {
            target_actor_id: ActorUniqueID(0),
            target_runtime_id: ActorRuntimeID(0),
            position: Vec3 {
                x: 0.0,
                y: 6.0,
                z: 0.0,
            },
            rotation: Vec2 { x: 270.0, y: 90.0 },
            settings: LevelSettings {
                seed: 999,
                spawn_settings: SpawnSettings {
                    biome_type: SpawnBiomeType::Default,
                    user_defined_biome_name: String::from("RandomBiome"),
                    dimension: Dimension::Overworld,
                },
                generator_type: GeneratorType::Overworld,
                game_type: GameType::Creative,
                is_hardcore_mode_enabled: false,
                game_difficulty: Difficulty::Peaceful,
                achievements_disabled: true,
                editor_world_type: EditorWorldType::NotEditor,
                is_created_in_editor: false,
                day_cycle_stop_time: -1,
                education_edition_offer: EducationEditionOffer::None,
                education_product_id: String::from(""),
                rain_level: 0.0,
                lightning_level: 0.0,
                has_confirmed_platform_locked_content: false,
                multiplayer_enabled: true,
                lan_broadcasting_enabled: true,
                xbox_live_broadcast_setting: GamePublishSetting::Public,
                platform_broadcast_setting: GamePublishSetting::Public,
                commands_enabled: false,
                texture_packs_required: false,
                experiments: Experiments {
                    experiments: vec![],
                    ever_toggled: false,
                },
                bonus_chest_enabled: false,
                starting_map_enabled: false,
                player_permissions: PlayerPermissionLevel::Member,
                server_chunk_tick_range: 4,
                locked_behaviour_pack: false,
                from_locked_template: false,
                from_template: false,
                only_spawn_v1_villagers: false,
                persona_disabled: false,
                custom_skins_disabled: false,
                emote_chat_muted: false,
                base_game_version: BaseGameVersion(String::from("1.21.71")),
                limited_world_width: 16,
                limited_world_depth: 16,
                edu_shared_uri_resource: EduSharedResourceUri {
                    button_name: String::from(""),
                    link_uri: String::from(""),
                },
                chat_restriction_level: ChatRestrictionLevel::None,
                disable_player_interactions: false,
                server_identifier: "".to_string(),
                server_world_identifier: "".to_string(),
                default_spawn_block_position: NetworkBlockPosition { x: 0, y: 0, z: 0 },
                is_exported_from_editor: false,
                education_features_enabled: false,
                rule_data: GameRulesChangedPacketData { rules_list: vec![] },
                locked_resource_pack: false,
                use_msa_gamer_tags: false,
                has_locked_template_settings: false,
                nether_type: false,
                override_force_experimental_gameplay: false,
                server_scenario_identifier: "".to_string(),
            },
            level_id: String::from("PowerCrabX"),
            level_name: String::from("Hello World"),
            template_content_identity: String::from(""),
            movement_settings: SyncedPlayerMovementSettings {
                authority_mode: ServerAuthMovementMode::ServerAuthoritative,
                rewind_history_size: 0,
                server_authoritative_block_breaking: true,
            },
            current_level_time: 0,
            enchantment_seed: 0,
            block_properties: Vec::new(),
            multiplayer_correlation_id: String::from(""),
            server_version: String::from("1.21.71"),
            player_property_data: nbtx::Value::Compound(HashMap::new()),
            world_template_id: Uuid::nil(),
            server_enabled_client_side_generation: false,
            block_network_ids_are_hashes: true,
            is_trial: false,
            enable_item_stack_net_manager: false,
            server_block_type_registry_checksum: 0,
            network_permissions: NetworkPermissions {
                server_auth_sound: false,
            },
            player_gamemode: Gamemode::Creative,
        };

        let mut playerlist = Vec::new();

        for name in login_data.iter() {
            match name {
                GamePackets::Login(data) => {
                    // playerlist.push(AddPlayerListEntry {
                    //     uuid: data.client_uuid,
                    //     target_actor_id: ActorUniqueID(0),
                    //     player_name: data.clone().username,
                    //     xbl_xuid: "".to_string(),
                    //     platform_chat_id: "".to_string(),
                    //     build_platform: BuildPlatform::Google,
                    //     serialized_skin: data.clone().skin,
                    //     is_teacher: false,
                    //     is_host: false,
                    //     is_sub_client: false,
                    //     is_trusted_skin: true,
                    // });
                    println!("Login: {:?}", data.connection_request.is_xbox_auth());
                }
                _ => {}
            }
        }

        let mut att: Vec<AttributeData> = Vec::new();
        att.push(AttributeData {
            min_value: 0.0,
            max_value: 340282346638528859811704183484516925440.00,
            current_value: 0.1,
            default_min_value: 0.0,
            default_max_value: 340282346638528859811704183484516925440.00,
            default_value: 0.1,
            name: String::from("minecraft:movement"),
            modifiers: Vec::new(),
        });

        self.connection
            .send(&[
                GamePackets::StartGame(packet1),
                GamePackets::ItemRegistry(ItemRegistryPacket { items: vec![] }),
                GamePackets::PlayerList(PlayerListPacket {
                    action: PlayerListPacketType::Add {
                        add_player_list: playerlist,
                    },
                }),
                GamePackets::UpdateAttributes(UpdateAttributesPacket {
                    target_runtime_id: ActorRuntimeID(0),
                    attribute_list: att,
                    tick: 0,
                }),
            ])
            .await
            .unwrap();
        println!("StartGame");

        let chunk_position_x = 0 >> 4;
        let chunk_position_z = 0 >> 4;
        let chunk_radius = 30;
        for x in -chunk_radius..chunk_radius {
            for z in -chunk_radius..chunk_radius {
                let mut chunk = LevelChunkPacket {
                    chunk_position: ChunkPos {
                        x: 0 >> 4,
                        z: 0 >> 4,
                    },
                    dimension_id: 0,
                    cache_enabled: false,
                    cache_blobs: vec![],
                    serialized_chunk_data: "".to_string(),
                };
                chunk.chunk_position.x = chunk_position_x + x;
                chunk.chunk_position.z = chunk_position_z + z;
                self.connection
                    .send(&[GamePackets::LevelChunk(chunk)])
                    .await
                    .unwrap();
            }
        }

        self.connection
            .send(&[GamePackets::PlayStatus(PlayStatusPacket {
                status: PlayStatusType::PlayerSpawn,
            })])
            .await
            .unwrap();
        println!("PlayStatusPacket (PlayerSpawn)");

        let time_end = Instant::now();

        println!("{:?}", time_end.duration_since(time_start));

        loop {
            let res = self.connection.recv().await;

            if let Ok(packet) = res {
                for (packet) in packet.iter() {
                    match packet {
                        GamePackets::PlayerAuthInput(data) => {
                            // println!("PlayerAuthInput: {:?}", data);
                        }
                        _ => {
                            println!("packet: {:?}", packet);
                        }
                    }
                }
            } else {
                match res {
                    Ok(_) => {}
                    Err(err) => match err {
                        TransportError(err) => match err {
                            TransportLayerError::RakNetError(_) => {
                                println!("close connection raknet closed");
                                break;
                            }
                            _ => break,
                        },
                        ConnectionError::ProtoCodecError(_)
                        | ConnectionError::ConnectionClosed
                        | ConnectionError::IOError(_) => break,
                    },
                }
            }
        }
    }
}

use std::collections::HashMap;
use crate::network::connection::bedrock_session::BedrockSession;
use bedrockrs::proto::encryption::Encryption;
use bedrockrs::proto::v662::enums::{ConnectionFailReason, Difficulty, Dimension, EditorWorldType, EducationEditionOffer, GamePublishSetting, GameType, Gamemode, GeneratorType, PlayerPermissionLevel, ServerAuthMovementMode};
use bedrockrs::proto::v662::packets::{LevelChunkPacket, ServerToClientHandshakePacket};
use bedrockrs::proto::v662::types::{ActorRuntimeID, ActorUniqueID, GameRulesChangedPacketData, NetworkBlockPosition, SyncedPlayerMovementSettings};
use bedrockrs::proto::v729::packets::login::LoginPacket;
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
use bedrockrs::proto::v748::packets::{DisconnectPacket, DisconnectPacketMessage, ResourcePackStackPacket, UpdateAttributesPacket};
use bedrockrs::proto::v748::types::LevelSettings;
use bedrockrs::proto::v766::enums::PlayerListPacketType;
use bedrockrs::proto::v766::packets::{PlayerListPacket, ResourcePacksInfoPacket};
use bedrockrs::proto::v776::packets::{ItemRegistryPacket, StartGamePacket};
use bedrockrs::proto::v785::gamepackets::GamePackets;
use p384::ecdsa::SigningKey;
use rand_core::OsRng;
use uuid::Uuid;
use vek::{Vec2, Vec3};

pub async fn handle(mut session: &mut BedrockSession, packet_data: &LoginPacket) {
    let chain_data = &packet_data.connection_request;
    println!("ChainData: {:?}", chain_data);
    //mock
    let must_xbox = true;
    if (!chain_data.is_xbox_auth() && must_xbox) {
        session.send(&[
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
            .await;
        session.close().await;
        return;
    }

    let client_key = Encryption::parse_der_public_key(&chain_data.get_identity_public_key()).ok();

    // Generate ephemeral keypair for encryption
    let (encryption_secret, _encryption_public) = Encryption::create_key_pair();

    // Generate 16-byte token
    let token = Encryption::generate_random_token();

    // Derive secret key for encryption
    let secret_key = Encryption::get_secret_key(&encryption_secret, &client_key.unwrap(), &token);

    // Create handshake JWT
    let signing_key = SigningKey::random(&mut OsRng);
    let jwt = Encryption::create_handshake_jwt(&signing_key, &token).unwrap();
    println!("jwt: {:?}", jwt);
    // session.send(&[
    //     GamePackets::ServerToClientHandshake(ServerToClientHandshakePacket {
    //         handshake_web_token: jwt.clone(),
    //     })
    // ]).await;
    session.send(&[
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
        .await;
    println!("send {:?}", jwt);

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

    session
        .send(&[
            GamePackets::StartGame(packet1),
            GamePackets::ItemRegistry(ItemRegistryPacket { items: vec![] })
        ])
        .await;
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
            session
                .send(&[GamePackets::LevelChunk(chunk)])
                .await
        }
    }

    session
        .send(&[GamePackets::PlayStatus(PlayStatusPacket {
            status: PlayStatusType::PlayerSpawn,
        })])
        .await;
    println!("PlayStatusPacket (PlayerSpawn)");
}
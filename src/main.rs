mod entity;
mod level;
mod server;

use std::collections::HashMap;
use std::env;
use bedrockrs::proto::connection::Connection;
use bedrockrs::proto::listener::Listener;
use bedrockrs::proto::compression::Compression;
use tokio::time::Instant;
use uuid::Uuid;
use vek::{Vec2, Vec3};
use bedrockrs::proto::v662::enums::{Difficulty, Dimension, EditorWorldType, EducationEditionOffer, GamePublishSetting, GameType, Gamemode, GeneratorType, PacketCompressionAlgorithm, PlayerPermissionLevel, ServerAuthMovementMode};
use bedrockrs::proto::v662::packets::{NetworkSettingsPacket};
use bedrockrs::proto::v662::types::{ActorRuntimeID, ActorUniqueID, GameRulesChangedPacketData, NetworkBlockPosition, SyncedPlayerMovementSettings};
use bedrockrs::proto::v729::packets::play_status::PlayStatusPacket;
use bedrockrs::proto::v729::types::base_game_version::BaseGameVersion;
use bedrockrs::proto::v729::types::chat_restriction_level::ChatRestrictionLevel;
use bedrockrs::proto::v729::types::edu_shared_uri_resource::EduSharedResourceUri;
use bedrockrs::proto::v729::types::experiments::Experiments;
use bedrockrs::proto::v729::types::network_permissions::NetworkPermissions;
use bedrockrs::proto::v729::types::play_status::PlayStatusType;
use bedrockrs::proto::v729::types::spawn_biome_type::SpawnBiomeType;
use bedrockrs::proto::v729::types::spawn_settings::SpawnSettings;
use bedrockrs::proto::v748::packets::{ResourcePackStackPacket};
use bedrockrs::proto::v748::types::LevelSettings;
use bedrockrs::proto::v766::packets::ResourcePacksInfoPacket;
use bedrockrs::proto::v776::gamepackets::GamePackets;
use bedrockrs::proto::v776::helper::ProtoHelperV776;
use bedrockrs::proto::v776::packets::{ItemRegistryPacket, StartGamePacket};
use crate::server::Server;

#[tokio::main]
async fn main() {
    let data_path = env::current_dir().unwrap().to_string_lossy().to_string();
    let server = Server::new(&data_path, &data_path);
    println!("Server started");
    // let mut listener = Listener::new_raknet(
    //     "PowerCrabX!!!".to_string(),
    //     "Hello World".to_string(),
    //     "1.0".to_string(),
    //     20,
    //     10,
    //     "127.0.0.1:19132".parse().unwrap(),
    //     false,
    // )
    //     .await
    //     .unwrap();
    //
    // listener.start().await.unwrap();
    //
    // loop {
    //     let conn = listener.accept().await.unwrap();
    //
    //     tokio::spawn(async move {
    //         handle_login(conn).await;
    //     });
    // }
}

async fn handle_login(mut conn: Connection<ProtoHelperV776>) {
    let time_start = Instant::now();

    // NetworkSettingsRequest
    conn.recv().await.unwrap();
    println!("NetworkSettingsRequest");

    let compression = Compression::None;

    // NetworkSettings
    conn.send(&[GamePackets::NetworkSettings(NetworkSettingsPacket {
        compression_threshold: 1,
        compression_algorithm: PacketCompressionAlgorithm::None,
        client_throttle_enabled: false,
        client_throttle_threshold: 0,
        client_throttle_scalar: 0.0,
    })])
        .await
        .unwrap();
    println!("NetworkSettings");

    conn.compression = Some(compression);

    // Login
    println!("Login data {:?}", conn.recv().await.unwrap());

    conn.send(&[
        GamePackets::PlaySatus(PlayStatusPacket {
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
            xbox_live_broadcast_setting: GamePublishSetting::NoMultiPlay,
            platform_broadcast_setting: GamePublishSetting::NoMultiPlay,
            commands_enabled: false,
            texture_packs_required: false,
            experiments: Experiments {
                experiments: vec![],
                ever_toggled: false,
            },
            bonus_chest_enabled: false,
            starting_map_enabled: false,
            player_permissions: PlayerPermissionLevel::Member,
            server_chunk_tick_range: 0,
            locked_behaviour_pack: false,
            from_locked_template: false,
            from_template: false,
            only_spawn_v1_villagers: false,
            persona_disabled: false,
            custom_skins_disabled: false,
            emote_chat_muted: false,
            base_game_version: BaseGameVersion(String::from("1.21.0")),
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
            default_spawn_block_position: NetworkBlockPosition {
                x: 0,
                y: 0,
                z: 0,
            },
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
            rewind_history_size: 3200,
            server_authoritative_block_breaking: false,
        },
        current_level_time: 0,
        enchantment_seed: 0,
        block_properties: vec![],
        multiplayer_correlation_id: String::from(""),
        server_version: String::from("1.21.71"),
        player_property_data: nbtx::Value::Compound(HashMap::new()),
        world_template_id: Uuid::nil(),
        server_enabled_client_side_generation: false,
        block_network_ids_are_hashes: false,
        is_trial: false,
        enable_item_stack_net_manager: false,
        server_block_type_registry_checksum: 0,
        network_permissions: NetworkPermissions { server_auth_sound: false },
        player_gamemode: Gamemode::Creative,
    };

    conn.send(&[
        GamePackets::StartGame(packet1),
        GamePackets::ItemRegistryPacket(ItemRegistryPacket { items: vec![] })
    ]).await.unwrap();
    println!("StartGame");

    conn.send(&[GamePackets::PlaySatus(PlayStatusPacket {
        status: PlayStatusType::PlayerSpawn,
    })])
        .await.unwrap();
    println!("PlayStatusPacket (PlayerSpawn)");

    let time_end = Instant::now();

    println!("{:?}", time_end.duration_since(time_start));

    loop {
        let res = conn.recv().await;

        if let Ok(packet) = res {
            println!("{:?}", packet);
        }
    }
}

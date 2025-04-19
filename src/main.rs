mod entity;
mod level;
mod server;
mod utils;
mod network;
mod resource_pack;

use std::collections::HashMap;
use std::env;
use std::ops::Deref;
use std::process::exit;
use std::sync::Arc;
use std::thread::spawn;
use bedrockrs::addon::Addon;
use bedrockrs::addon::resource::ResourcePack;
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
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use crate::server::Server;
use log::{error, info};
use log4rs;
use crate::network::Network;
use crate::utils::sem_version::SemVersion;

async fn test() {
    // let age = 30;
    // let person = Person { name: String::from("tset") };
    //
    // let print = async {
    //   println!("Age: {}", age);
    //     println!("name: {}", &person.name);
    // };
    //
    //
    //
    // tokio::spawn(print).await.unwrap();
    //
    // println!("Age: {}", age);
    // println!("Name: {}", person.name);
    //
    // println!("finish");
}

static TITLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));
static ANSI: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default())
        .unwrap_or_else(|err| {
            eprintln!("Failed to initialize log4rs: {}", err);
            exit(1);
        });
    let data_path = env::current_dir().unwrap().to_string_lossy().to_string();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(6)
        .enable_all()
        .build()
        .unwrap_or_else(|err| {
            error!("Failed to create Tokio-Runtime, Err: {err:?}");
            exit(1)
        });

    runtime.block_on(async {
        if *TITLE.lock().await {
            info!("Starting PowerCrabX...")
        }

        let server = Arc::new(Mutex::new(Server::default()));

        Network::new(server.lock().await).await;

        server.lock().await.start().await.unwrap_or_else(|err| {
            error!("{}", err);
        });

        if *TITLE.lock().await {
            info!("Stopping PowerCrabX...")
        }
        info!("Stopped.")
    });

    // println!("Server started ", server);

}

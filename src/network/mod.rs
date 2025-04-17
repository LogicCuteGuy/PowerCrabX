mod connection;
mod process;
mod protocol_info;

use bedrockrs::proto::connection::Connection;
use bedrockrs::proto::listener::Listener;
use bedrockrs::proto::v785::helper::ProtoHelperV785;
use crate::network::connection::bedrock_session::BedrockSession;
use crate::server::Server;

pub struct Network {
    pub name: String,
}

impl Network {
    pub async fn new(server: tokio::sync::MutexGuard<'_, Server>) -> Network {
        tokio::spawn(async move {
            let mut listener = Listener::new_raknet(
                "PowerCrabX!!!".to_string(),
                "Hello World".to_string(),
                "1.0".to_string(),
                20,
                10,
                "127.0.0.1:19132".parse().unwrap(),
                false,
            )
                .await
                .unwrap();

            listener.start().await.unwrap();
            loop {
                let mut conn: Connection<ProtoHelperV785> = listener.accept().await.unwrap();
                println!("{}", conn.get_ip_address().await.unwrap());
                tokio::spawn(async move {
                    println!("spawn Task");
                    let mut bedrock_session = BedrockSession::new(conn);
                    bedrock_session.start().await;
                    // handle_login(conn).await;
                });
            }
        });

        Network {
            name: String::new(),
        }
    }
}
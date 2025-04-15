use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;

pub struct Server {
    lunchtime: u128,
    data_path: String,
}

impl Server {
    pub async fn new(file_path: &str, data_path: &str) -> Server {
        let lunchtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();


        let world_path_str = format!("{}/worlds/", data_path);
        let players_path_str = format!("{}/players/", data_path);

        let world_path = Path::new(&world_path_str);
        let players_path = Path::new(&players_path_str);

        ensure_dir_exists(world_path).await;
        ensure_dir_exists(players_path).await;

        // let data_path = fs::canonicalize(data_path);

        Server {
            lunchtime,
            data_path: String::new(),
        }
    }
}

async fn ensure_dir_exists(path: &Path) {
    if !path.exists() {
        if let Err(e) = fs::create_dir_all(path).await {
            eprintln!("Failed to create directory {:?}: {}", path, e);
        } else {
            println!("Directory {:?} created.", path);
        }
    }
}
use serde::{Deserialize, Serialize};
use serde_yaml::{Value, to_string};
use std::collections::{HashMap, HashSet};
use std::fs::{File, create_dir_all};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use log::{info, error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigSection {
    data: HashMap<String, Value>,
}

impl ConfigSection {
    pub fn new() -> Self {
        ConfigSection {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn get_all(&self) -> &HashMap<String, Value> {
        &self.data
    }

    pub fn keys(&self) -> HashSet<String> {
        self.data.keys().cloned().collect()
    }
}

pub struct Config {
    config: Arc<Mutex<ConfigSection>>,
    file: Option<String>,
    file_type: ConfigType,
}

#[derive(Debug, PartialEq)]
pub enum ConfigType {
    Properties,
    Json,
    Yaml,
    Enum,
    Detect,
}

impl Config {
    pub fn new(file: Option<String>, file_type: ConfigType) -> Self {
        Config {
            config: Arc::new(Mutex::new(ConfigSection::new())),
            file,
            file_type: if file_type == ConfigType::Detect { ConfigType::Yaml } else { file_type },
        }
    }

    pub fn load(&mut self) -> Result<(), String> {
        let file = self.file.clone().ok_or("File path is not provided.")?;
        let path = Path::new(&file);
        if !path.exists() {
            create_dir_all(path.parent().unwrap()).unwrap();
            File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
            self.save()?;
            return Ok(());
        }

        let mut content = String::new();
        let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        file.read_to_string(&mut content).map_err(|e| format!("Failed to read file: {}", e))?;

        match self.file_type {
            ConfigType::Properties => self.parse_properties(&content),
            ConfigType::Json => self.parse_json(&content),
            ConfigType::Yaml => self.parse_yaml(&content),
            _ => Err("Unsupported config type".to_string()),
        }
    }

    fn parse_properties(&mut self, content: &str) {
        // Parse properties format (key=value)
        for line in content.lines() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].to_string();
                let value = Value::String(parts[1].to_string());
                self.config.lock().unwrap().insert(key, value);
            }
        }
    }

    fn parse_json(&mut self, content: &str) {
        let config_section: HashMap<String, Value> = serde_json::from_str(content).unwrap();
        self.config.lock().unwrap().data = config_section;
    }

    fn parse_yaml(&mut self, content: &str) {
        let config_section: HashMap<String, Value> = serde_yaml::from_str(content).unwrap();
        self.config.lock().unwrap().data = config_section;
    }

    pub fn save(&self) -> Result<(), String> {
        let file = self.file.clone().ok_or("File path is not provided.")?;
        let path = Path::new(&file);
        let mut file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;

        let content = match self.file_type {
            ConfigType::Properties => self.write_properties(),
            ConfigType::Json => serde_json::to_string_pretty(&*self.config.lock().unwrap()).unwrap(),
            ConfigType::Yaml => to_string(&*self.config.lock().unwrap()).unwrap(),
            _ => return Err("Unsupported config type".to_string()),
        };

        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write file: {}", e))?;
        Ok(())
    }

    fn write_properties(&self) -> String {
        let mut content = String::new();
        for (key, value) in &*self.config.lock().unwrap().get_all() {
            let value_str = match value {
                Value::Bool(b) => if *b { "on" } else { "off" },
                Value::String(s) => &s,
                _ => "unknown",
            };
            content.push_str(&format!("{}={}\n", key, value_str));
        }
        content
    }

    pub fn set(&mut self, key: &str, value: Value) {
        self.config.lock().unwrap().insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.config.lock().unwrap().get(key).cloned()
    }

    pub fn get_keys(&self) -> HashSet<String> {
        self.config.lock().unwrap().keys()
    }
}
use anyhow::Result;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::new());
}

macro_rules! config {
    {
        $($val_name:ident: $ty:ty => $getter:ident, $setter:ident)*
    } => {
        /// Note: Fields should only be added to the config.
        /// Also let's assume only a single instance of synth is running
        #[derive(Serialize, Deserialize, Default)]
        struct Config {
            $(
                #[serde(skip_serializing_if = "Option::is_none")]
                $val_name: Option<$ty>,
            )*
        }

        $(
            #[allow(dead_code)]
            pub fn $setter($val_name: $ty) -> Result<()> {
                let mut config = CONFIG.lock().unwrap();
                config.$val_name = Some($val_name);
                config.save()
            }
            #[allow(dead_code)]
            pub fn $getter() -> Option<$ty> {
                CONFIG.lock().unwrap().$val_name.clone()
            }
        )*

    }
}

config! {
    uuid: String => get_uuid, set_uuid
    telemetry_enabled: bool => get_telemetry_enabled, set_telemetry_enabled
    seen_versions: HashSet<String> => get_seen_versions, set_seen_versions
    version_check_delay: DateTime<Utc> => get_version_check_delay, set_version_check_delay
}

impl Config {
    fn new() -> Self {
        Self::from_file().unwrap_or_else(|_| Config::default())
    }

    fn from_file() -> Result<Self> {
        let file_contents = std::fs::read_to_string(Self::file_path()?)?;
        let config = serde_json::from_str(&file_contents)?;
        Ok(config)
    }

    fn file_path() -> Result<PathBuf> {
        Ok(Self::synth_config_dir()?.join("config.json"))
    }

    fn synth_config_dir() -> Result<PathBuf> {
        let synth_config_dir = dirs::config_dir()
            .ok_or_else(|| {
                anyhow!("Could not find a configuration directory. Your operating system may not be supported.")
            })?;
        Ok(synth_config_dir.join("synth"))
    }

    fn save(&self) -> Result<()> {
        // Create config dir if it doesn't exist
        let config_dir = Self::synth_config_dir()?;
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }

        // Save the config
        let mut config_file_path = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Self::file_path()?)?;

        serde_json::to_writer_pretty(&mut config_file_path, &self)?;

        Ok(())
    }
}

use std::{env, sync::LazyLock};
use figment::{Figment, providers::{Format, Toml, Json, Env}};
use log::LevelFilter;
use serde::Deserialize;

// use crate::pkg::figment_string;

// #[derive(Deserialize, Debug)]
// pub struct Redis {
//     pub host: String,
//     #[serde(deserialize_with = "figment_string::deserialize_as_string")]
//     pub password: String,
//     #[serde(deserialize_with = "figment_string::deserialize_as_string")]
//     pub port: String,
// }

// impl Clone for Redis
// {
//     fn clone(&self) -> Self {
//         Redis {
//             host: self.host.clone(),
//             password: self.password.clone(),
//             port: self.port.clone(),
//         }
//     }
// }

// #[derive(Deserialize, Debug)]
// pub struct Gemini {
//     #[serde(rename = "apikey")]
//     pub api_key:String,
//     #[serde(rename = "program", deserialize_with = "figment_string::deserialize_as_string")]
//     pub program_number: String,
//     #[serde(rename = "enabled")]
//     pub is_active: bool,
//     pub model: String,
// }
// 
// impl Clone for Gemini
// {
//     fn clone(&self) -> Self {
//         Gemini {
//             api_key: self.api_key.clone(),
//             model: self.model.clone(),
//             program_number: self.program_number.clone(),
//             is_active: self.is_active.clone(),
//         }
//     }
// }

// #[derive(Deserialize, Debug)]
// pub struct TTS {
//     #[serde(rename = "apikey")]
//     pub api_key:String,
//     #[serde(rename = "enabled")]
//     pub is_active: bool,
// }
// 
// impl Clone for TTS
// {
//     fn clone(&self) -> Self {
//         TTS {
//             api_key: self.api_key.clone(),
//             is_active: self.is_active.clone(),
//         }
//     }
// }

// #[derive(Deserialize, Debug)]
// pub struct JobParameters {
//     pub name: String,
//     pub schedule: String,
// }

// impl Clone for JobParameters
// {
//     fn clone(&self) -> Self {
//         JobParameters {
//             name: self.name.clone(),
//             schedule: self.schedule.clone(),
//         }
//     }
// }

// #[derive(Deserialize, Debug)]
// pub struct JobConfig {
//     pub daily_data: JobParameters,
//     pub clean_data: JobParameters,
// }

// impl Clone for JobConfig
// {
//     fn clone(&self) -> Self {
//         JobConfig {
//             daily_data: self.daily_data.clone(),
//             clean_data: self.clean_data.clone(),
//         }
//     }
// }

#[derive(Deserialize, Debug)]
pub struct Config {
    pub version: String,
    /// The port to listen on
    pub port: u16,

    pub host: String,
    /// Base URL for the OpenAI API (e.g., https://api.openai.com/v1)
    pub base_url: String,
    /// API keys for authenticating with OpenAI
    pub api_keys: Vec<String>,
    /// Access keys for authenticating to Proxy
    pub access_keys: Vec<String>,

    pub log_level: Option<LevelFilter>,
    
    pub acivity_logging_path: Option<String>,
    // pub gemini: Gemini,
    // pub tts: TTS,
    // pub jobs: JobConfig,
}

impl Clone for Config
{
    fn clone(&self) -> Self {
        Config {
            version: self.version.clone(),
            api_keys: self.api_keys.clone(),
            access_keys: self.access_keys.clone(),
            log_level: self.log_level.clone(),
            port: self.port.clone(),
            host: self.host.clone(),
            base_url: self.base_url.clone(),
            acivity_logging_path: self.acivity_logging_path.clone(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let file_env = env::var("ENV").ok();
        let file_name = match file_env {
            Some(file_sufix) => format!("config.{}", file_sufix),
            None => "config".to_string(),
        };
        let figment = Figment::new()
        .merge(Toml::file(format!("{}.toml", file_name)))
        .merge(Env::prefixed("PROXY_").split("_"))
        .join(Json::file(format!("{}.json", file_name)));
        
        match figment.extract() {
            Ok(config) => config,
            Err(e) => {
                log::error!("{}", e);
                panic!("{}", e);
            }
        }
    }
}

impl Config {
    pub fn instance() -> Config {
        static STATIC_INSTANCE: LazyLock<Config> = LazyLock::new(Config::default);
        STATIC_INSTANCE.clone()
    }
}

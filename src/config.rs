extern crate figment;

use serde::{Deserialize, Serialize};

use figment::{
    providers::{Env, Format, Json, Serialized},
    Figment,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub windows: Vec<WindowConfig>,
}

impl Config {
    pub fn load() -> Result<Config, figment::Error> {
        let figment: Figment = Figment::from(Serialized::defaults(Config::default()))
            .merge(Json::file("Settings.json"))
            .merge(Env::prefixed("APP_"));

        let config: Config = figment.extract()?;

        Ok(config)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WindowConfig {
    pub always_on_top: bool,
    pub window_title: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            windows: vec![WindowConfig::default()],
        }
    }
}

impl Default for WindowConfig {
    fn default() -> WindowConfig {
        WindowConfig {
            always_on_top: false,
            window_title: String::from("TestWindow"),
            url: String::from("http://google.com"),
            height: 400,
            width: 300,
        }
    }
}

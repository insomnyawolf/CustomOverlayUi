extern crate figment;

use serde::{Deserialize, Serialize};

use figment::{
    providers::{Env, Format, Json, Serialized},
    Figment,
};
use wry::application::dpi::{PhysicalSize, PhysicalPosition};

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
pub struct Config {
    pub enable_debugging: bool, // Currently Unused
    pub enable_experimental_features: bool,
    pub windows: Vec<WindowConfig>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            enable_debugging: false, // Currently Unused
            enable_experimental_features: false,
            windows: vec![WindowConfig::default()],
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WindowConfig {
    pub window_title: String,
    pub url: String,
    pub always_on_top: bool,
    pub fullscreen: bool,
    pub resizable: bool,
    pub size: PhysicalSize<u32>,
    pub position: PhysicalPosition<u32>
}

impl Default for WindowConfig {
    fn default() -> WindowConfig {
        WindowConfig {
            window_title: String::from("SampleWindow"),
            url: String::from("https://github.com/insomnyawolf"),
            always_on_top: false,
            fullscreen: false,
            resizable: false,
            size: PhysicalSize {
                height: 400,
                width: 300,
            },
            position: PhysicalPosition {
                x: 0,
                y: 0,
            }
        }
    }
}

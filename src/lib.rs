#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod app_config;

pub use app_config::create_app_config_from_env;

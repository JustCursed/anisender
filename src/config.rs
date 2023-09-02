use serde::Deserialize;
use serde_json::from_str;
use std::fs::read_to_string;

pub const EMBED_COLOR: u32 = 0xffc857;

const CONFIG_PATH: &str = "/home/funkyra/.config/config.json";

#[derive(Deserialize)]
pub struct Config {
	pub db: Database,
	pub token: String,
	pub handler_cfg: HandlerConfig,
	pub other: Other, // pub permissions: CommandsPerms,
}

#[derive(Deserialize)]
pub struct Database {
	pub url: String,
	pub username: String,
	pub password: String,
	pub database: String,
	pub namespace: String,
}

#[derive(Deserialize)]
pub struct HandlerConfig {
	pub guild_id: u64,
	pub streaming: Streaming,
	pub system_load: SystemLoadConfig,
	pub changer_status: ChangerStatus,
}

#[derive(Deserialize)]
pub struct Other {
	pub sites: Vec<String>,
	pub delete_duration: u64,
	pub cool_down_sender: u64,
}

#[derive(Deserialize)]
pub struct Streaming {
	pub name: String,
	pub link: String,
	pub enable: bool,
}

#[derive(Deserialize)]
pub struct SystemLoadConfig {
	pub enable: bool,
	pub channel_id: u64,
}

#[derive(Deserialize)]
pub struct ChangerStatus {
	pub enable: bool,
	pub message: String,
}

// #[derive(Deserialize, Debug)]
// pub struct CommandsPerms {
// 	pub system_stats: Vec<String>
// }

lazy_static::lazy_static! {
	pub static ref cfg: Config = from_str(
		read_to_string(CONFIG_PATH).as_ref().unwrap()
	).unwrap();
}

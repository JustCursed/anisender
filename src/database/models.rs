use serde::{Deserialize, Serialize};

pub enum Models {
	Add(Users),
}

// #[derive(Deserialize)]
// pub struct Site {
// 	service_name: String,
// 	urn: Vec<Urn>,
// }
//
// #[derive(Deserialize)]
// struct Urn {
// 	urn_name: String,
// 	last_series: u16,
// 	users: Vec<u32>,
// }
//
// #[derive(Serialize, Debug)]
// pub struct SerSite {
// 	pub service_name: &'static str,
// 	pub urn: Vec<SerUrn>,
// }
//
// #[derive(Serialize, Debug)]
// pub struct SerUrn {
// 	pub urn_name: &'static str,
// 	pub last_series: u16,
// 	pub users: Vec<u32>,
// }
//
// #[derive(Serialize)]
// pub struct CheckDB {
// 	pub name: String,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Users {
	pub id: u64,
	pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Anime {
	pub url: String,
	pub last_series: u32,
	pub time_last_series: u32,
}

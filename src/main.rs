#![allow(non_upper_case_globals)]
extern crate reqwest;
extern crate sys_info;

use crate::database::client::setup;

use config::cfg;
use serde::{Deserialize, Serialize};
use serenity::prelude::*;
use surrealdb::sql::Thing;

mod commands;
mod config;
mod database;
mod event_handler;
mod sites;

// use crate::database::db::init;

#[derive(Deserialize, Serialize, Debug)]
struct Cg {
	pub url: String,
}

// fn d() {
// 	let mut headers = HeaderMap::new();
// 	headers.insert("NS", "myapplication".parse().unwrap());
// 	headers.insert("DB", "myapplication".parse().unwrap());
// 	headers.insert("Accept", "application/json".parse().unwrap());
// 	headers.insert(
// 		"Content-Type",
// 		"application/x-www-form-urlencoded".parse().unwrap(),
// 	);
//
// 	let client = reqwest::blocking::Client::builder()
// 		.redirect(reqwest::redirect::Policy::none())
// 		.build()
// 		.unwrap();
// 	let res = client
// 		.post("http://localhost:8000/sql")
// 		.basic_auth("root", Some("root"))
// 		.headers(headers)
// 		.body("SELECT * FROM person WHERE age > 18")
// 		.send()
// 		.unwrap()
// 		.json::<Vec<Cg>>()
// 		.unwrap();
// 	println!("{:?}", res[0].result);
// }

#[derive(Debug, Serialize, Deserialize)]
struct Name {
	first: String,
	last: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
	title: String,
	name: Name,
	marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
	marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
	#[allow(dead_code)]
	id: Thing,
}

#[tokio::main]
async fn main() -> serenity::Result<()> {
	// let doc = Html::parse_document(
	// 	&get_html("https://amedia.online/837-patriotizm-moriarti-2/episode/10/seriya-onlayn.html")
	// 		.await,
	// );

	setup().await.expect("Failed to setup database");

	Client::builder(
		&cfg.read().unwrap().token,
		GatewayIntents::DIRECT_MESSAGES | GatewayIntents::GUILDS,
	)
	.event_handler(event_handler::Handler {
		streaming: cfg.read().unwrap().handler_cfg.streaming.enable,
		system_load: cfg.read().unwrap().handler_cfg.system_load.enable,
		changer_status: cfg.read().unwrap().handler_cfg.changer_status.enable,
	})
	.await?
	.start()
	.await?;

	// let db = Surreal::new::<Ws>("0.0.0.0:8000").await?;
	//
	// db.signin(Root {
	// 	username: "root",
	// 	password: "root",
	// })
	// 	.await?;
	//
	// db.use_ns("t").use_db("td").await?;
	//
	// db
	// 	.query("CREATE $table SET id = $id, url = 'da'")
	// 	.bind(("table", "pizda"))
	// 	.await?;
	//
	// println!("pizda");
	//
	// let mut d: Response = db.query("SELECT * FROM type::table($table) WHERE url = $url")
	// 	.bind(("table", "pizda"))
	// 	.bind(("url", "da"))
	// 	// .bind(("url", "amedia.online"))
	// 	.await?;
	//
	// println!("{:?}", d.take::<Vec<Cg>>(0)?);

	Ok(())
}
// https://amedia.online/6-van-pis/episode/226/seriya-onlayn.html
// https://amedia.online/837-patriotizm-moriarti-2/episode/6/seriya-onlayn.html

async fn get_html(link: &str) -> String {
	reqwest::get(link)
		.await
		.expect("Failed to send request")
		.text()
		.await
		.expect("Failed to get html")
}

// TODO:
// 1. make a parser
// 2. make a command

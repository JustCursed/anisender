use serenity::client::Context;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

use crate::config::cfg;
use crate::database::client::DB;
use crate::database::models::Anime;

pub fn start(ctx: Arc<Context>) {
	if cfg.other.cool_down_sender < 120 {
		eprintln!("Too low value for cool down");
		std::process::abort();
	}

	tokio::spawn(async move {
		loop {
			check(ctx.clone()).await.expect("TODO: panic message");
			sleep(Duration::from_secs(cfg.other.cool_down_sender)).await;
		}
	});
}

async fn check(ctx: Arc<Context>) -> surrealdb::Result<()> {
	for anime in DB.select::<Vec<Anime>>("url").await? {
		match anime.url.as_str() {
			"" => println!(),
			_ => println!(),
		}
	}

	Ok(())
}

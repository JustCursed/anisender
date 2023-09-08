use crate::config::cfg;

use chrono::Local;
use serenity::{client::Context, model::gateway::Activity};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub async fn start(ctx: Arc<Context>) {
	let msg = match cfg.handler_cfg.changer_status.message.as_str() {
		"time" => || Local::now().to_rfc2822(),
		_ => || cfg.handler_cfg.changer_status.message.to_owned(),
	};

	tokio::spawn(async move {
		loop {
			ctx.set_activity(Activity::watching(msg())).await;
			sleep(Duration::from_secs(2)).await;
		}
	});
}

use serenity::{
	client::Context,
	futures::StreamExt,
	model::id::{ChannelId, MessageId},
};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

use crate::config::{cfg, EMBED_COLOR};

pub async fn start(ctx: Arc<Context>) {
	let msg_id = preparation(ctx.to_owned()).await;

	tokio::spawn(async move {
		loop {
			log_system_load(ctx.to_owned(), msg_id).await;
			sleep(Duration::from_secs(4)).await;
		}
	});
}

async fn preparation(ctx: Arc<Context>) -> MessageId {
	delete_all_messages(ctx.to_owned()).await;

	ChannelId(cfg.handler_cfg.system_load.channel_id)
		.say(&ctx, "system_load preparation...")
		.await
		.expect("Failed to system_load preparation");

	ChannelId(cfg.handler_cfg.system_load.channel_id)
		.messages_iter(&ctx)
		.boxed()
		.next()
		.await
		.unwrap()
		.unwrap()
		.id
}

async fn delete_all_messages(ctx: Arc<Context>) {
	let mut messages = ChannelId(cfg.handler_cfg.system_load.channel_id)
		.messages_iter(&ctx)
		.boxed();

	while let Some(msg_result) = messages.next().await {
		match msg_result {
			Ok(msg) => msg.delete(&ctx).await.expect("Failed to delete msg"),
			Err(err) => eprintln!("Uh oh! Error: {}", err),
		}
	}
}

async fn log_system_load(ctx: Arc<Context>, msg_id: MessageId) {
	let cpu_load = sys_info::loadavg().unwrap();
	let mem = sys_info::mem_info().unwrap();
	let disk = sys_info::disk_info().unwrap();

	let message = ChannelId(cfg.handler_cfg.system_load.channel_id)
		.edit_message(ctx, msg_id, |msg| {
			msg.content("").embed(|e| {
				e.color(EMBED_COLOR)
					.title("System resource load")
					// cpu
					.field("", "**CPU load average within:**", false)
					.field("1 minute", format!("{:.2}%", cpu_load.one * 10.0), true)
					.field("5 minutes", format!("{:.2}%", cpu_load.five * 10.0), true)
					.field("15 minutes", format!("{:.2}%", cpu_load.fifteen * 10.0), true)
					// memory
					.field("", "**Memory stats:**", false)
					.field("All", format!("{:.2} GB", mem.total as f32 / 1000000.0), true)
					.field("Avail", format!("{:.2} GB", mem.avail as f32 / 1000000.0), true)
					.field("Free", format!("{:.2} MB", mem.free as f32 / 1000.0), true)
					// disk info
					.field("", "**Disk info**", false)
					.field("All", format!("{:.0} GB", disk.total as f32 / 1000000.0), true)
					.field("Free", format!("{:.2} GB", disk.free as f32 / 1000000.0), true)
					// footer
					.footer(|f| {
						f.text("Created by JustCursed ©")
							.icon_url("https://avatars.githubusercontent.com/u/69715980?v=64")
					})
			})
		})
		.await;
	if let Err(why) = message {
		eprintln!("Error sending message: {:?}", why);
	};
}

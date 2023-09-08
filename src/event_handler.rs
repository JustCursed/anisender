mod cache_ready;
mod interaction_create;

use crate::commands;
use crate::config::cfg;
use crate::event_handler::interaction_create::application_command::execute_command;

use cache_ready::{anime_checker, changer_status, system_load};
use std::sync::Arc;

use serenity::{
	builder::CreateEmbed,
	model::{
		application::interaction::{Interaction, InteractionResponseType},
		channel::Message,
		gateway::{Activity, Ready},
		id::{ChannelId, GuildId, UserId},
		user::User,
	},
	prelude::*,
};

pub struct Handler {
	pub streaming: bool,
	pub system_load: bool,
	pub changer_status: bool,
}

#[async_trait::async_trait]
impl EventHandler for Handler {
	async fn cache_ready(&self, context: Context, _guilds: Vec<GuildId>) {
		println!("Cache built successfully!");

		let ctx = Arc::new(context);

		if self.system_load {
			system_load::start(ctx.to_owned()).await;
		}

		if self.changer_status {
			changer_status::start(ctx.to_owned()).await;
		}

		if self.streaming {
			ctx.set_activity(Activity::streaming(
				&cfg.read().unwrap().handler_cfg.streaming.name,
				&cfg.read().unwrap().handler_cfg.streaming.link,
			))
			.await;
		}

		anime_checker::start(ctx.to_owned());

		println!("All services has been loaded");

		// let embed = CreateEmbed::default()
		// 	.title("anime name")
		// 	.description("description")
		// 	.image("https://i.imgur.com/wtV1qvf.jpeg")
		// 	.footer(|f| f.text("Created by JustCursed ©"));
		//
		// UserId(461539277713571840)
		// 	.create_dm_channel(&ctx)
		// 	.await
		// 	.unwrap()
		// 	.send_message(&ctx, |m| {
		// 		m.embed(|e| {
		// 			e.title("anime name")
		// 				.description("description")
		// 				.footer(|f| {
		// 					f.text("Created by JustCursed ©")
		// 						.icon_url("https://avatars.githubusercontent.com/u/69715980?v=64")
		// 				})
		// 		})
		// 	})
		// 	.await
		// 	.expect("TODO: panic message");
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);

		for guild in ready.guilds {
			GuildId::set_application_commands(&guild.id, &ctx.http, |cmds| {
				cmds.create_application_command(|cmd| commands::id::register(cmd))
					.create_application_command(|cmd| commands::listen::register(cmd))
			})
			.await
			.expect("Failed create application command");
		}
	}

	async fn interaction_create(&self, ctx: Context, inter: Interaction) {
		if let Interaction::ApplicationCommand(command) = &inter {
			execute_command(command, ctx, &inter).await
		}
	}
}

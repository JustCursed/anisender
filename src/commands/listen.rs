use std::sync::Arc;

use serenity::client::Context;
use serenity::{
	builder::CreateApplicationCommand,
	model::prelude::{
		command::CommandOptionType,
		interaction::{
			application_command::{CommandDataOption, CommandDataOptionValue},
			Interaction
		},
	},
};

use crate::config::cfg;
use crate::database::client::DB;
use crate::database::models::Users;

pub async fn run(options: &[CommandDataOption], inter: &Interaction) -> &'static str {
	let option = options
		.get(0)
		.expect("Expected user option")
		.resolved
		.as_ref()
		.expect("Expected user object");

	if let CommandDataOptionValue::String(arg) = option {
		let site = match arg.starts_with("http") {
			true => arg.split('/').collect::<Vec<&str>>()[2],
			false => arg.split('/').collect::<Vec<&str>>()[0],
		}
		.to_string();

		return match cfg.other.sites.contains(&site) {
			true => {
				// DB.create(("user")).await;

				// inter.application_command().unwrap().user.id

				"Anime successfully added for listening!"
			}
			false => "Sorry, I'm not supporting this site yet :(",
		};
		// return format!("{:?}", arg.split('/').collect::<Vec<&str>>()[2]);
	}
	"Please provide a valid user"
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("listen")
		.description("add anime for listening")
		.create_option(|option| {
			option
				.name("link")
				.description("link site with anime")
				.kind(CommandOptionType::String)
				.required(true)
		})
}

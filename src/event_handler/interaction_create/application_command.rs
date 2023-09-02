use crate::commands;

use serenity::{
	client::Context,
	model::application::interaction::{
		application_command::ApplicationCommandInteraction, Interaction, InteractionResponseType,
	},
};

pub async fn execute_command(
	command: &ApplicationCommandInteraction,
	ctx: Context,
	inter: &Interaction,
) {
	let content = match command.data.name.as_str() {
		"id" => commands::id::run(&command.data.options),
		"listen" => commands::listen::run(&command.data.options, inter).await,
		_ => "not implemented :(",
	};

	command
		.create_interaction_response(&ctx.http, |response| {
			response
				.kind(InteractionResponseType::ChannelMessageWithSource)
				.interaction_response_data(|message| message.content(content))
		})
		.await
		.unwrap();
}

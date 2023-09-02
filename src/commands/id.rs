use serenity::{
	builder::CreateApplicationCommand,
	model::prelude::{
		command::CommandOptionType,
		interaction::application_command::{CommandDataOption, CommandDataOptionValue},
	},
};

pub fn run(options: &[CommandDataOption]) -> &'static str {
	let option = options
		.get(0)
		.expect("Expected user option")
		.resolved
		.as_ref()
		.expect("Expected user object");

	if let CommandDataOptionValue::User(user, _member) = option {
		println!("{}", user.id);
		return "";
	}
	"Please provide a valid user"
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("id")
		.description("Get a user id")
		.create_option(|option| {
			option
				.name("id")
				.description("The user to lookup")
				.kind(CommandOptionType::User)
				.required(true)
		})
}

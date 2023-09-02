use serenity::{
	builder::CreateApplicationCommand,
	model::prelude::{
		command::CommandOptionType,
		interaction::application_command::{CommandDataOption, CommandDataOptionValue},
	},
};

pub fn run(options: &[CommandDataOption]) -> String {
	// let option = options
	// 	.get(0)
	// 	.expect("Expected user option")
	// 	.resolved
	// 	.as_ref()
	// 	.expect("Expected user object");
    //
	// if let CommandDataOptionValue::User(user, _member) = option {
	// 	format!("{}'s id is {}", user.tag(), user.id)
	// } else {
	// 	"Please provide a valid user".to_string()
	// }

	return "".to_string();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("stats")
		.description("Getting system statistics")
}

use serenity::builder::{ CreateCommand, CreateCommandOption};
use serenity::model::application::{ ResolvedOption, ResolvedValue, CommandOptionType };

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = options.first()
    {
        format!("{}'s id is {}", user.tag(), user.id)
    }
    else {
        "please provide a vaild user.".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("id")
        .description("id command")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "id",
                "the user to query id"
            )
            .required(true)
        )
}
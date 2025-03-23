use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "id".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("id")
        .description("id command")
}
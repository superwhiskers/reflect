use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandError, CommandResult,
    },
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
use std::str::FromStr;

group!({
    name: "Utility",
    options: {
        description: "General commands for doing things with the bot",
    },
    commands: [setup],
});

// TODO(superwhiskers): add utility commands
#[command]
#[description = "Sets up the mirror channel in the server"]
#[only_in(guilds)]
#[required_permissions(ADMINISTRATOR)]
pub fn setup(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    let channel_id;
    match arguments.is_empty() {
        true => channel_id = message.channel_id,
        false => match arguments.parse::<u64>() {
            Ok(parsed_channel_id) => channel_id = ChannelId(parsed_channel_id),
            Err(_) => match arguments.parse::<String>() {
                Ok(parsed_channel_id) => match ChannelId::from_str(
                    parsed_channel_id
                        .trim_end_matches('>')
                        .trim_start_matches("<#"),
                ) {
                    Ok(parsed_channel_id) => channel_id = parsed_channel_id,
                    Err(error_message) => {
                        message.channel_id.say(&context.http, "Unable to parse the provided argument as either a channel id or a channel mention")?;
                        return Err(CommandError(format!("{:?}", error_message)));
                    }
                },
                Err(error_message) => {
                    message.channel_id.say(&context.http, "Unable to parse the provided argument as either a channel id or a channel mention")?;
                    return Err(CommandError(format!("{:?}", error_message)));
                }
            },
        },
    }

    message.channel_id.say(
        &context.http,
        format!(
            "Setting up the mirror channel in this server at <#{}>",
            channel_id.0
        ),
    )?;

    Ok(())
}

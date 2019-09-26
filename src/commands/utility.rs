use log::{debug, error};
use r2d2_redis::redis::Commands;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
use std::str::FromStr;

use crate::types;

group!({
    name: "Utility",
    options: {
        description: "General commands for doing things with the bot",
    },
    commands: [setup],
});

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
                    Err(_) => {
                        message.channel_id.say(&context.http, "**Error:** Unable to parse the provided argument as either a channel id or a channel mention")?;
                        return Ok(());
                    }
                },
                Err(_) => {
                    message.channel_id.say(&context.http, "**Error:** Unable to parse the provided argument as either a channel id or a channel mention")?;
                    return Ok(());
                }
            },
        },
    }

    let mut status_message = message.channel_id.say(
        &context.http,
        format!(
            "Setting up the mirror channel in this server at <#{}>",
            channel_id.0,
        ),
    )?;

    // get a database connection
    let data = context.data.read();
    let mut database = match data.get::<types::Database>() {
        Some(database) => database.get()?,
        None => panic!("the database wasn't initialized and placed into the data TypeMap (this is a severe bug)"),
    };

    // erase the existing channel from the channel set if necessary
    debug!(
        "checking for an existing mirror channel in guild {}",
        message.guild_id.unwrap().0
    );
    match database.hget::<u64, &str, Option<u64>>(message.guild_id.unwrap().0, "mirror_channel") {
        Ok(channel) => {
            if let Some(channel) = channel {
                debug!(
                    "found an existing mirror channel for guild {} at {}",
                    message.guild_id.unwrap().0,
                    channel
                );
                status_message.edit(&context, |m| m.content(format!("Detected existing mirror channel at <#{}>. Erasing it from the `messages` set before updating", channel)))?;

                // remove the existing channel
                match database.srem::<&str, u64, u8>("channels", channel) {
                    Ok(_) => (),
                    Err(message) => {
                        error!("unable to remove an existing miror channel from the messages set: {:?}", message);
                        return Ok(());
                    }
                }
            }
        }
        Err(message) => {
            error!(
                "unable to check for an existing mirror channel in a guild: {:?}",
                message
            );
            return Ok(());
        }
    }

    // update the guild's top-level hash
    match database.hset::<u64, &str, u64, u8>(
        message.guild_id.unwrap().0,
        "mirror_channel",
        channel_id.0,
    ) {
        Ok(_) => (),
        Err(message) => {
            error!(
                "unable to update the guild hash to reflect the new mirror channel: {:?}",
                message
            );
            return Ok(());
        }
    }

    // update the channel set
    match database.sadd::<&str, u64, u8>("channels", channel_id.0) {
        Ok(_) => (),
        Err(message) => {
            error!(
                "unable to update the channel set to contain the new mirror channel: {:?}",
                message
            );
            return Ok(());
        }
    }

    status_message.edit(&context, |m| {
        m.content(format!(
            "Finished. Try talking in <#{}> and see if anyone replies!",
            channel_id.0
        ))
    })?;

    Ok(())
}

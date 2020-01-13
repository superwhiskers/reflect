use log::{debug, error};
use r2d2_redis::redis::{self, Commands};
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
use std::str::FromStr;

use crate::{get_db_handle, say_error};

#[group]
#[description = "General commands for doing things with the bot"]
#[commands(enable, disable)]
pub struct Utility;

#[command]
#[description = "Enables a mirror channel in the server"]
#[only_in(guilds)]
#[required_permissions(ADMINISTRATOR)]
pub fn enable(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    // figure out which channel the caller wants us to convert into a mirror channel
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
                        say_error!(message, context, "Unable to parse the provided argument to a channel id!");
                        return Ok(());
                    }
                },
                Err(_) => {
                    say_error!(message, context, "Unable to parse the provided argument to a channel id!");
                    return Ok(());
                }
            },
        },
    }

    // send the initial status message
    let mut status_message = message.channel_id.say(
        &context.http,
        format!(
            "Enabling the mirror channel in this server at <#{}>",
            channel_id.0,
        ),
    )?;

    let mut database = get_db_handle!(context.data.read());

    // the guild id is needed in a lot of places. predefine it instead of unwrapping each time we
    // need it
    let guild_id = message.guild_id.unwrap().0;

    // erase the existing channel from the channel set if necessary
    debug!(
        "checking for an existing mirror channel in guild {}",
        guild_id,
    );
    match database.hget::<u64, &str, Option<u64>>(guild_id, "mirror_channel") {
        Ok(chan) => {
            if let Some(chan) = chan {
                debug!(
                    "found an existing mirror channel for guild {} at {}",
                    guild_id, chan,
                );

                // remove the existing channel from the channels set
                match database.srem::<&str, u64, u8>("channels", chan) {
                    Ok(_) => (),
                    Err(msg) => {
                        error!("unable to remove an existing miror channel from the messages set: {:?}", msg);
                        say_error!(message, context, "Unable to remove the existing mirror channel from the channels set!");
                        return Ok(());
                    }
                }
            }
        }
        Err(msg) => {
            error!(
                "unable to check for an existing mirror channel in a guild: {:?}",
                msg
            );
            say_error!(message, context, "Unable to check for an existing mirror channel!");
            return Ok(());
        }
    }

    // update the guild's top-level hash
    match database.hset::<u64, &str, u64, bool>(guild_id, "mirror_channel", channel_id.0) {
        Ok(_) => (),
        Err(msg) => {
            error!(
                "unable to update the guild hash to reflect the new mirror channel: {:?}",
                msg
            );
            say_error!(message, context, "Unable to update the channel id in the guild hash!");
            return Ok(());
        }
    }

    // update the channel set
    match database.sadd::<&str, u64, bool>("channels", channel_id.0) {
        Ok(_) => (),
        Err(msg) => {
            error!(
                "unable to update the channel set to contain the new mirror channel: {:?}",
                msg
            );
            say_error!(message, context, "Unable to update the channel id in the channel set!");
            return Ok(());
        }
    }

    // update the status message one last time
    status_message.edit(&context, |m| {
        m.content(format!(
            "Finished. Try talking in <#{}> and see if anyone replies!",
            channel_id.0
        ))
    })?;

    Ok(())
}

#[command]
#[description = "Disables the mirror channel in your server"]
#[only_in(guilds)]
#[required_permissions(ADMINISTRATOR)]
pub fn disable(context: &mut Context, message: &Message) -> CommandResult {
    // send the initial status message
    let mut status_message = message
        .channel_id
        .say(&context.http, "Disabling your server's mirror channel")?;

    // get a database connection
    let mut database = get_db_handle!(context.data.read());

    // the guild id is needed in a lot of places. predefine it instead of unwrapping each time we
    // need it
    let guild_id = message.guild_id.unwrap().0;

    match database.hget::<u64, &str, Option<u64>>(guild_id, "mirror_channel") {
        Ok(chan) => {
            if let Some(chan) = chan {
                debug!(
                    "found a mirror channel for guild {} at {}",
                    guild_id, chan
                );

                // remove the channel from the channels set
                match database.srem::<&str, u64, bool>("channels", chan) {
                    Ok(_) => (),
                    Err(msg) => {
                        error!("unable to disable an existing miror channel from the channels set: {:?}", msg);
                        say_error!(message, context, "Unable to remove the channel id from the channels set!");
                        return Ok(());
                    }
                }

                // remove the guild from the top-level key-value store
                match redis::cmd("UNLINK")
                    .arg(guild_id)
                    .query::<u8>(&mut (*database))
                {
                    Ok(_) => (),
                    Err(msg) => {
                        error!(
                            "unable to remove the guild {}'s top-level hash from redis: {:?}",
                            guild_id, msg
                        );
                        say_error!(message, context, "Unable to remove the channel id from the guild hash!");
                        return Ok(());
                    }
                }

                status_message.edit(&context, |m| {
                    m.content(format!(
                        "Disabled your server's mirror channel at <#{}>",
                        chan,
                    ))
                })?;
            } else {
                status_message.edit(&context, |m| {
                    m.content("Your server has no mirror channel to disable")
                })?;
            }
        }
        Err(msg) => {
            error!(
                "unable to check for an existing mirror channel in a guild: {:?}",
                msg
            );
            say_error!(message, context, "Unable to check for an existing mirror channel!");
            return Ok(());
        }
    }

    Ok(())
}

//
//  reflect - a global mirror channel for discord guilds
//  Copyright (C) superwhiskers <whiskerdev@protonmail.com> 2020
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Affero General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU Affero General Public License for more details.
//
//  You should have received a copy of the GNU Affero General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

use log::{debug, error};
use r2d2_redis::redis::{self, Commands};
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{
        channel::{Channel, Message},
        id::{ChannelId, UserId},
    },
    prelude::*,
};
use std::str::FromStr;

use crate::{
    colors, commands::checks::ADMIN_CHECK, get_db_handle, say, say_error, utils::resolve_user,
};

#[group]
#[description = "General commands for doing things with the bot"]
#[commands(enable, disable, user, notify)]
pub struct Utility;

#[command]
#[description = "Look up information about a user"]
pub fn user(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    debug!("looking up user information");

    let user = match if arguments.len() == 0 {
        message.author.id
    } else {
        UserId(match resolve_user(context, message, arguments) {
            Ok(ids) => {
                if ids.len() > 1 {
                    say_error!(
                        message,
                        context,
                        "Usercache listing is not implemented yet!"
                    );
                    return Ok(());
                } else if ids.len() == 0 {
                    say_error!(message, context, "No user could be found!");
                    return Ok(());
                }
                ids[0]
            }
            Err(msg) => {
                say_error!(message, context, msg);
                return Ok(());
            }
        })
    }
    .to_user(&context)
    {
        Ok(user) => user,
        Err(msg) => {
            error!("unable to convert userid -> user: {}", msg);
            say_error!(message, context, "Unable to get the provided user!");
            return Ok(());
        }
    };

    debug!("got user to look up: {:?}", user);

    message.channel_id.send_message(&context, |m| {
        m.embed(|e| {
            e.title(user.tag())
                .description(format!("Showing user information for {}", user.name))
                .thumbnail(user.face())
                .fields(vec![
                    (
                        "Information",
                        format!("**ID:** {}\n**Bot:** {}\n", user.id.0, user.bot),
                        false,
                    ),
                    ("Mirror Channel Servers", "Unimplemented".into(), false),
                ])
                .color(colors::PRIMARY)
        })
    })?;

    Ok(())
}

#[command]
#[description = "Broadcast a notification to all servers that the bot is in"]
#[checks(Admin)]
pub fn notify(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.len() == 0 {
        say_error!(message, context, "No message was provided!");
        return Ok(());
    }

    let mut database = get_db_handle!(context.data.read());

    let channel_iterator = match database.sscan::<&str, u64>("channels") {
        Ok(iter) => iter,
        Err(msg) => {
            error!("unable to iterate over the mirror channels: {:?}", msg);
            return Ok(());
        }
    };

    for channel in channel_iterator {
        if channel == message.channel_id.0 {
            continue;
        }

        let channel = match ChannelId(channel).to_channel(&context) {
            Ok(chan) => match chan {
                Channel::Guild(chan) => chan,
                _ => {
                    error!("channel is not a guild channel");
                    continue;
                }
            },
            Err(msg) => {
                error!("unable to get channel: {:?}", msg);
                continue;
            }
        };
        let channel = channel.read();

        let guild = match channel.guild_id.to_partial_guild(&context) {
            Ok(guild) => guild,
            Err(msg) => {
                error!("unable to get partial guild: {:?}", msg);
                continue;
            }
        };

        match channel.say(
            &context,
            format!(
                "**Notification (<@{}>):** {}",
                guild.owner_id.0,
                arguments.message()
            ),
        ) {
            Ok(_) => (),
            Err(msg) => {
                error!("unable to say message: {:?}", msg);
                continue;
            }
        }
    }

    say!(
        message,
        context,
        "Notification",
        "Your notification has been sent."
    );

    return Ok(());
}

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
                        say_error!(
                            message,
                            context,
                            "Unable to parse the provided argument to a channel id!"
                        );
                        return Ok(());
                    }
                },
                Err(_) => {
                    say_error!(
                        message,
                        context,
                        "Unable to parse the provided argument to a channel id!"
                    );
                    return Ok(());
                }
            },
        },
    }

    // send the initial status message
    let mut status_message = message.channel_id.send_message(&context, |m| {
        m.embed(|e| {
            e.title("Enabling")
                .description(format!(
                    "Enabling the mirror channel in this server at <#{}>",
                    channel_id.0
                ))
                .color(colors::PRIMARY)
        })
    })?;

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
                        say_error!(
                            message,
                            context,
                            "Unable to remove the existing mirror channel from the channels set!"
                        );
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
            say_error!(
                message,
                context,
                "Unable to check for an existing mirror channel!"
            );
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
            say_error!(
                message,
                context,
                "Unable to update the channel id in the guild hash!"
            );
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
            say_error!(
                message,
                context,
                "Unable to update the channel id in the channel set!"
            );
            return Ok(());
        }
    }

    // update the status message one last time
    status_message.edit(&context, |m| {
        m.embed(|e| {
            e.title("Enabling")
                .description(format!(
                    "Finished. Try talking in <#{}> and see if anyone replies!",
                    channel_id.0
                ))
                .color(colors::PRIMARY)
        })
    })?;

    Ok(())
}

#[command]
#[description = "Disables the mirror channel in your server"]
#[only_in(guilds)]
#[required_permissions(ADMINISTRATOR)]
pub fn disable(context: &mut Context, message: &Message) -> CommandResult {
    // send the initial status message
    let mut status_message = message.channel_id.send_message(&context, |m| {
        m.embed(|e| {
            e.title("Disabling")
                .description("Disabling your server's mirror channel")
                .color(colors::PRIMARY)
        })
    })?;

    // get a database connection
    let mut database = get_db_handle!(context.data.read());

    // the guild id is needed in a lot of places. predefine it instead of unwrapping each time we
    // need it
    let guild_id = message.guild_id.unwrap().0;

    match database.hget::<u64, &str, Option<u64>>(guild_id, "mirror_channel") {
        Ok(chan) => {
            if let Some(chan) = chan {
                debug!("found a mirror channel for guild {} at {}", guild_id, chan);

                // remove the channel from the channels set
                match database.srem::<&str, u64, bool>("channels", chan) {
                    Ok(_) => (),
                    Err(msg) => {
                        error!("unable to disable an existing miror channel from the channels set: {:?}", msg);
                        say_error!(
                            message,
                            context,
                            "Unable to remove the channel id from the channels set!"
                        );
                        return Ok(());
                    }
                }

                status_message.edit(&context, |m| {
                    m.embed(|e| {
                        e.title("Disabling")
                            .description(format!(
                                "Disabled your server's mirror channel at <#{}>",
                                chan
                            ))
                            .color(colors::PRIMARY)
                    })
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
            say_error!(
                message,
                context,
                "Unable to check for an existing mirror channel!"
            );
            return Ok(());
        }
    }

    // remove the guild from the top-level key-value store
    match redis::cmd("UNLINK")
        .arg(guild_id)
        .query::<u64>(&mut (*database))
    {
        Ok(_) => (),
        Err(msg) => {
            error!(
                "unable to remove the guild {}'s top-level hash from redis: {:?}",
                guild_id, msg
            );
            say_error!(
                message,
                context,
                "Unable to remove the channel id from the guild hash!"
            );
            return Ok(());
        }
    }

    Ok(())
}

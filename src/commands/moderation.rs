use log::{debug, error};
use r2d2_redis::redis::{self, Commands};
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

use crate::{commands::checks::ADMIN_CHECK, get_db_handle};

group!({
    name: "Moderation",
    options: {
        description: "Commands used to moderate the mirror channel",
    },
    commands: [ban, unban],
});

// TODO(superwhiskers): consider moving the user id grabbing into a macro

#[command]
#[description = "Bans a user from the global mirror channel"]
#[only_in(guilds)]
#[checks(Admin)]
pub fn ban(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    // TODO(superwhiskers): fully implement after adding usercache stuff
    let user_id;
    match arguments.is_empty() {
        true => {
            message
                .channel_id
                .say(&context.http, "**Error:** No user was provided to ban")?;
            return Ok(());
        }
        false => match arguments.parse::<u64>() {
            Ok(parsed_user_id) => user_id = parsed_user_id,
            Err(_) => match arguments.parse::<String>() {
                Ok(parsed_user_mention) => match parsed_user_mention
                    .trim_end_matches('>')
                    .trim_start_matches("<@")
                    .trim_start_matches("!")
                    .parse::<u64>()
                {
                    Ok(parsed_user_id) => user_id = parsed_user_id,
                    Err(_) => {
                        message.channel_id.say(&context.http, "**Error:** Usercache support has not been implemented yet. Unable to retrieve user id from provided argument")?;
                        return Ok(());
                    }
                },
                Err(_) => {
                    message.channel_id.say(&context.http, "**Error:** Usercache support has not been implemented yet. Unable to retrieve user id from provided argument")?;
                    return Ok(());
                }
            },
        },
    }

    debug!("banning user id {} from the global mirror channel", user_id);

    let mut database = get_db_handle!(context.data.read());

    // check if the user we're banning is an admin
    match database.sismember::<&str, u64, bool>("admins", user_id) {
        Ok(admin) => {
            if admin {
                message
                    .channel_id
                    .say(&context.http, "**Error:** You cannot ban an admin!")?;
                return Ok(());
            }
        }
        Err(message) => {
            error!(
                "unable to check if the provided user id is of an admin: {:?}",
                message
            );
            return Ok(());
        }
    }

    // ban them
    match database.sadd::<&str, u64, bool>("banned", user_id) {
        Ok(_) => (),
        Err(message) => {
            error!(
                "unable to update the banned user set to contain a new banned member: {:?}",
                message
            );
            return Ok(());
        }
    }

    // provide a form of feedback
    message.channel_id.say(
        &context.http,
        format!(
            "Successfully banned `{}` from the global mirror channel!",
            user_id
        ),
    )?;

    Ok(())
}

#[command]
#[description = "Unbans a user from the global mirror channel"]
#[only_in(guilds)]
#[checks(Admin)]
pub fn unban(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    // TODO(superwhiskers): fully implement after adding usercache stuff
    let user_id;
    match arguments.is_empty() {
        true => {
            message
                .channel_id
                .say(&context.http, "**Error:** No user was provided to unban")?;
            return Ok(());
        }
        false => match arguments.parse::<u64>() {
            Ok(parsed_user_id) => user_id = parsed_user_id,
            Err(_) => match arguments.parse::<String>() {
                Ok(parsed_user_mention) => match parsed_user_mention
                    .trim_end_matches('>')
                    .trim_start_matches("<@")
                    .trim_start_matches("!")
                    .parse::<u64>()
                {
                    Ok(parsed_user_id) => user_id = parsed_user_id,
                    Err(_) => {
                        message.channel_id.say(&context.http, "**Error:** Usercache support has not been implemented yet. Unable to retrieve user id from provided argument")?;
                        return Ok(());
                    }
                },
                Err(_) => {
                    message.channel_id.say(&context.http, "**Error:** Usercache support has not been implemented yet. Unable to retrieve user id from provided argument")?;
                    return Ok(());
                }
            },
        },
    }

    debug!(
        "unbanning user id {} from the global mirror channel",
        user_id
    );

    let mut database = get_db_handle!(context.data.read());

    // unban them
    match database.srem::<&str, u64, bool>("banned", user_id) {
        Ok(_) => (),
        Err(message) => {
            error!(
                "unable to update the banned user set to remove a new banned member: {:?}",
                message
            );
            return Ok(());
        }
    }

    // provide a form of feedback
    message.channel_id.say(
        &context.http,
        format!(
            "Successfully unbanned `{}` from the global mirror channel!",
            user_id
        ),
    )?;

    Ok(())
}

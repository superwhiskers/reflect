use log::{debug, error};
use r2d2_redis::redis::Commands;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

use crate::{commands::checks::ADMIN_CHECK, get_db_handle, say_error, utils::resolve_user};

group!({
    name: "Moderation",
    options: {
        description: "Commands used to moderate the mirror channel",
    },
    commands: [ban, unban],
});

#[command]
#[description = "Bans a user from the global mirror channel"]
#[only_in(guilds)]
#[checks(Admin)]
pub fn ban(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    let user_id = match resolve_user(context, message, arguments) {
        Ok(id) => id,
        Err(msg) => {
            say_error!(message, context, msg);
            return Ok(());
        }
    };

    debug!("banning user id {} from the global mirror channel", user_id);

    let mut database = get_db_handle!(context.data.read());

    // check if the user we're banning is an admin
    match database.sismember::<&str, u64, bool>("admins", user_id) {
        Ok(admin) => {
            if admin {
                say_error!(message, context, "You cannot ban an admin!");
                return Ok(());
            }
        }
        Err(msg) => {
            error!(
                "unable to check if the provided user id is of an admin: {:?}",
                msg
            );
            say_error!(message, context, "Unable to check if the user is an admin!");
            return Ok(());
        }
    }

    // ban them
    match database.sadd::<&str, u64, bool>("banned", user_id) {
        Ok(_) => (),
        Err(msg) => {
            error!(
                "unable to update the banned user set to contain a new banned user: {:?}",
                msg
            );
            say_error!(
                message,
                context,
                "Unable to add the user to the banned users' list!"
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
    let user_id = match resolve_user(context, message, arguments) {
        Ok(id) => id,
        Err(msg) => {
            say_error!(message, context, msg);
            return Ok(());
        }
    };

    debug!(
        "unbanning user id {} from the global mirror channel",
        user_id
    );

    let mut database = get_db_handle!(context.data.read());

    // unban them
    match database.srem::<&str, u64, bool>("banned", user_id) {
        Ok(_) => (),
        Err(msg) => {
            error!(
                "unable to update the banned user set to remove a new banned user: {:?}",
                msg
            );
            say_error!(
                message,
                context,
                "Unable to remove the user from the banned users' list!"
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

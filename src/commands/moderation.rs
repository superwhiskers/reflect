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

#[group]
#[description = "Commands used to moderate the mirror channel"]
#[commands(ban, unban)]
pub struct Moderation;

#[command]
#[description = "Bans a user from the global mirror channel"]
#[only_in(guilds)]
#[checks(Admin)]
pub fn ban(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    let user_id = match resolve_user(context, message, arguments) {
        Ok(ids) => {
            if ids.len() > 1 {
                say_error!(
                    message,
                    context,
                    "Usercache listing is not implemented yet!"
                );
                return Ok(());
            } else if ids.len() == 0 {
                say_error!(
                    message,
                    context,
                    "No user could be found!"
                );
                return Ok(());
            }
            ids[0]
        }
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
        Ok(ids) => {
            if ids.len() > 1 {
                say_error!(
                    message,
                    context,
                    "Usercache listing is not implemented yet!"
                );
                return Ok(());
            } else if ids.len() == 0 {
                say_error!(
                    message,
                    context,
                    "No user could be found!"
                );
                return Ok(());
            }
            ids[0]
        }
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

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
use crate::get_db_handle;
use r2d2_redis::redis::Commands;
use serenity::{framework::standard::Args, model::channel::Message, prelude::*};

#[inline(always)]
fn usercache_lookup(
    context: &mut Context,
    _: &Message,
    arguments: Args,
) -> Result<Vec<u64>, &'static str> {
    debug!("performing a usercache lookup of \"{}\"", arguments.message());

    let mut database = get_db_handle!(context.data.read());

    let mut key = String::from("usercache-");
    key.push_str(arguments.message());

    match database.smembers::<&str, Vec<u64>>(&key) {
        Ok(cache) => return Ok(cache),
        Err(msg) => {
            error!("failed usercache lookup: {}", msg);
            return Err("Unable to lookup the provided user!");
        }
    }
}

/// resolves a user from the arguments passed to a command
/// returns an error message if something fails
pub fn resolve_user(
    context: &mut Context,
    message: &Message,
    arguments: Args,
) -> Result<Vec<u64>, &'static str> {
    match arguments.is_empty() {
        true => Err("No user was provided!"),
        false => match arguments.parse::<u64>() {
            Ok(parsed_user_id) => Ok(vec![parsed_user_id]),
            Err(_) => match arguments.parse::<String>() {
                Ok(parsed_user_mention) => match parsed_user_mention
                    .trim_end_matches('>')
                    .trim_start_matches("<@")
                    .trim_start_matches("!")
                    .parse::<u64>()
                {
                    Ok(parsed_user_id) => Ok(vec![parsed_user_id]),
                    Err(_) => usercache_lookup(context, message, arguments),
                },
                Err(_) => usercache_lookup(context, message, arguments),
            },
        },
    }
}

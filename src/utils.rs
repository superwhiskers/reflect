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

use serenity::{framework::standard::Args, model::channel::Message, prelude::*};

// TODO(superwhiskers): fully implement after adding usercache stuff
/// resolves a user from the arguments passed to a command
/// returns an error message if something fails
pub fn resolve_user(
    context: &mut Context,
    message: &Message,
    arguments: Args,
) -> Result<u64, &'static str> {
    macro_rules! usercache_lookup {
        ($context:ident, $message:ident, $arguments:ident) => {
            Err("Usercache support has not been implemented yet. Unable to retrieve user id from the provided argument")
        };
    }

    match arguments.is_empty() {
        true => Err("No user was provided!"),
        false => match arguments.parse::<u64>() {
            Ok(parsed_user_id) => Ok(parsed_user_id),
            Err(_) => match arguments.parse::<String>() {
                Ok(parsed_user_mention) => match parsed_user_mention
                    .trim_end_matches('>')
                    .trim_start_matches("<@")
                    .trim_start_matches("!")
                    .parse::<u64>()
                {
                    Ok(parsed_user_id) => Ok(parsed_user_id),
                    Err(_) => usercache_lookup!(context, message, arguments),
                },
                Err(_) => usercache_lookup!(context, message, arguments),
            },
        },
    }
}

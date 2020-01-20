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

use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};
use std::collections::HashSet;

//use super::super::colors;

#[help]
#[individual_command_tip = "To get more information about a command or a command group, pass the name of the command or group as an argument"]
#[command_not_found_text = "{} is not a command"]
#[max_levenshtein_distance(3)]
#[lacking_permissions("Strike")]
pub fn help(
    context: &mut Context,
    message: &Message,
    arguments: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    // TODO(superwhiskers): kinda annyoing that i can't provide an exact color to this w/o
    // manually defining the command and associated structures - do that later when i verify
    // everything else works properly
    help_commands::with_embeds(context, message, arguments, help_options, groups, owners)
}

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

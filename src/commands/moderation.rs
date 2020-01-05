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

use crate::commands::checks::ADMIN_CHECK;

group!({
    name: "Moderation",
    options: {
        description: "Commands used to moderate the mirror channel",
    },
    commands: [ban],
});

#[command]
#[description = "Bans a user from the global mirror channel"]
#[only_in(guilds)]
#[checks(Admin)]
pub fn ban(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    // TODO(superwhiskers): implement
    Ok(())
}

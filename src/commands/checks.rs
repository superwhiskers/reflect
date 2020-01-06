use log::debug;
use r2d2_redis::redis::Commands;
use serenity::{
    framework::standard::{macros::check, Args, CheckResult, CommandOptions},
    model::channel::Message,
    prelude::*,
};

use crate::get_db_handle;

// check that verifies if the executing user is a bot admin
#[check]
#[name = "Admin"]
pub fn bot_administrator(
    context: &mut Context,
    message: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> CheckResult {
    debug!("checking if a user is an admin");

    // get a database connection first
    let mut database = get_db_handle!(context.data.read());

    // verify
    match database.sismember::<&str, u64, bool>("admins", message.author.id.0) {
        Ok(is_admin) => {
            if is_admin {
                return CheckResult::Success;
            }

            // let the caller know that they are not an admin
            if let Err(msg) = message
                .channel_id
                .say(&context.http, "**Error:** You are not an admin!")
            {
                return CheckResult::new_log(msg);
            }

            return CheckResult::new_log("user was not an admin");
        }
        Err(msg) => CheckResult::new_log(msg),
    }
}

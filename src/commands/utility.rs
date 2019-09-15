use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

group!({
    name: "Utility",
    commands: [setup],
});

// TODO(superwhiskers): add utility commands
#[command]
fn setup(context: &mut Context, message: &Message) -> CommandResult {
    message
        .channel_id
        .say(&context.http, "Setting up Reflect in your server...")?;

    Ok(())
}

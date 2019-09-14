use log::{debug, info};
use serenity::{
    model::gateway::{Activity, Ready},
    model::channel::Message,
    prelude::*,
};

use crate::config;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, context: Context, event: Ready) {
        info!(
            "logged in as {} on {} servers",
            event.user.tag(),
            event.guilds.len()
        );
        let data = context.data.read();
        let prefix = match data.get::<config::Configuration>() {
            Some(config) => &config.prefix,
            None => panic!("no configuration was stored inside of the data typemap (this is a severe bug)"),
        };
        context.set_activity(Activity::listening(
            format!("your conversations | {}", prefix).as_str(),
        ));
    }

    fn message(&self, _: Context, message: Message) {
        debug!("got a new message: {:?}", message);

        // TODO(superwhiskers): finish implementing the message handler
    }
}

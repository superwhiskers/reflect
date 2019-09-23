use log::{debug, error, info};
use r2d2_redis::redis::Commands;
use serenity::{
    model::channel::Message,
    model::gateway::{Activity, Ready},
    model::id::ChannelId,
    prelude::*,
};

use crate::types;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, context: Context, event: Ready) {
        debug!("got ready event: {:?}", event);
        info!(
            "logged in as {} on {} servers",
            event.user.tag(),
            event.guilds.len(),
        );
        let data = context.data.read();
        let prefix = match data.get::<types::Configuration>() {
            Some(config) => &config.prefix,
            None => panic!(
                "no configuration was stored inside of the data TypeMap (this is a severe bug)"
            ),
        };
        context.set_activity(Activity::listening(
            format!("your conversations | {}", prefix).as_str(),
        ));
    }

    fn message(&self, context: Context, message: Message) {
        debug!("got a new message: {:?}", message);

        // ignore bots
        if message.author.bot {
            return;
        }

        let data = context.data.read();
        let mut database = match data.get::<types::Database>() {
            Some(database) => database.get().expect("unable to retrieve connection from the connection pool (this shouldnt've timed out)"),
            None => panic!("the database wasn't initialized and placed into the data TypeMap (this is a severe bug)"),
        };

        debug!("checking if the message was sent inside of a mirror channel");
        match database.sismember::<&str, u64, u64>("channels", *message.channel_id.as_u64()) {
            Ok(mirror_chan) => {
                if mirror_chan == 0 {
                    return;
                }
            }
            Err(message) => {
                error!("unable to check if the recieved message was sent inside of a mirror channel: {:?}", message);
                return;
            }
        }

        debug!("checking if the user who sent the message is banned");
        match database.sismember::<&str, u64, u64>("banned", *message.author.id.as_u64()) {
            Ok(banned) => {
                if banned == 1 {
                    return;
                }
            }
            Err(message) => {
                error!("unable to check if a user is banned: {:?}", message);
                return;
            }
        }

        let mut content = message.author.tag();
        match database.sismember::<&str, u64, u64>("admins", *message.author.id.as_u64()) {
            Ok(admin) => {
                if admin == 1 {
                    content.push_str(" **(__admin__)**: ");
                } else {
                    content.push_str(": ");
                }
            }
            Err(message) => {
                error!(
                    "unable to check if a user is an admin. assuming they are not: {:?}",
                    message
                );
                content.push_str(": ");
            }
        }

        content.push_str(&message.content_safe(&context.cache));

        debug!("mirroring message");
        let channel_iterator = match database.sscan::<&str, u64>("channels") {
            Ok(iter) => iter,
            Err(message) => {
                error!("unable to iterate over the mirror channels: {:?}", message);
                return;
            }
        };

        for channel in channel_iterator {
            let channel = ChannelId(channel);
            if channel == message.channel_id {
                continue;
            }
            channel.send_message(&context.http, |m| {
                m.content(&content)
                // i don't feel like handling files yet. the bare minimum is enough for now
            });
        }
    }
}

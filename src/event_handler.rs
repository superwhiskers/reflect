use log::{debug, error, info};
use r2d2_redis::redis::Commands;
use serenity::{
    http::AttachmentType,
    model::channel::Message,
    model::gateway::{Activity, Ready},
    model::id::ChannelId,
    prelude::*,
};
use std::borrow::Cow;

use crate::{get_db_handle, types};

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
            Some(cfg) => &cfg.prefix,
            None => panic!(
                "no configuration was stored inside of the data TypeMap (this is a severe bug)"
            ),
        };
        context.set_activity(Activity::listening(
            format!("your conversations | {}", prefix).as_str(),
        ));
    }

    fn message(&self, context: Context, message: Message) {
        match message.guild_id {
            Some(id) => debug!(
                "got a new message from guild {} in channel {}",
                id.0, message.channel_id.0
            ),
            None => return,
        }

        // ignore bots
        if message.author.bot {
            return;
        }

        let mut database = get_db_handle!(context.data.read());

        match database.sismember::<&str, u64, bool>("channels", message.channel_id.0) {
            Ok(mirror_chan) => {
                if !mirror_chan {
                    return;
                }
            }
            Err(msg) => {
                error!("unable to check if the recieved message was sent inside of a mirror channel: {:?}", msg);
                return;
            }
        }

        match database.sismember::<&str, u64, bool>("banned", message.author.id.0) {
            Ok(banned) => {
                if banned {
                    return;
                }
            }
            Err(msg) => {
                error!("unable to check if a user is banned: {:?}", msg);
                return;
            }
        }

        let mut content = message.author.tag();
        match database.sismember::<&str, u64, bool>("admins", message.author.id.0) {
            Ok(admin) => {
                if admin {
                    content.push_str(" **(__admin__)**: ");
                } else {
                    content.push_str(": ");
                }
            }
            Err(msg) => {
                error!(
                    "unable to check if a user is an admin. assuming they are not: {:?}",
                    msg
                );
                content.push_str(": ");
            }
        }

        content.push_str(&message.content_safe(&context.cache));

        debug!("mirroring message");
        let channel_iterator = match database.sscan::<&str, u64>("channels") {
            Ok(iter) => iter,
            Err(msg) => {
                error!("unable to iterate over the mirror channels: {:?}", msg);
                return;
            }
        };

        let mut files: Vec<AttachmentType> = Vec::with_capacity(message.attachments.len());

        for attachment in message.attachments {
            match attachment.download() {
                Ok(data) => files.push(AttachmentType::Bytes {
                    data: Cow::Owned(data),
                    filename: attachment.filename.clone(),
                }),
                Err(msg) => {
                    error!("unable to download attachment from discord: {:?}", msg);
                    return;
                }
            }
        }

        for channel in channel_iterator {
            let channel = ChannelId(channel);
            if channel == message.channel_id {
                continue;
            }
            if let Err(msg) =
                channel.send_message(&context.http, |m| {
                    m.content(&content);
                    m.2 = files.clone();
                    m
                })
            {
                error!("unable to mirror message to discord: {:?}", msg);
            }
        }
    }
}

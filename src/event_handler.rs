use log::{debug, error, info};
use r2d2_redis::redis::Commands;
use serenity::{
    http::AttachmentType,
    model::channel::Message,
    model::gateway::{Activity, Ready},
    model::id::ChannelId,
    prelude::*,
};
use std::default::Default;

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
        debug!(
            "got a new message from guild {} in channel {}",
            message.guild_id.unwrap().0,
            message.channel_id.0
        );

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
            Err(message) => {
                error!("unable to check if the recieved message was sent inside of a mirror channel: {:?}", message);
                return;
            }
        }

        match database.sismember::<&str, u64, bool>("banned", message.author.id.0) {
            Ok(banned) => {
                if banned {
                    return;
                }
            }
            Err(message) => {
                error!("unable to check if a user is banned: {:?}", message);
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

        let attachments_count = message.attachments.len();
        let mut file_data: Vec<(Vec<u8>, &str)> = Vec::with_capacity(attachments_count);
        let mut files = Vec::with_capacity(attachments_count);

        file_data.resize_with(attachments_count, Default::default);
        files.resize(attachments_count, AttachmentType::Bytes((&[], "")));

        for i in 0..attachments_count {
            match message.attachments[i].download() {
                Ok(data) => file_data[i] = (data, &message.attachments[i].filename),
                Err(message) => {
                    error!("unable to download attachment from discord: {:?}", message);
                    return;
                }
            }
        }

        for i in 0..attachments_count {
            files[i] = AttachmentType::Bytes((&file_data[i].0, file_data[i].1))
        }

        for channel in channel_iterator {
            let channel = ChannelId(channel);
            if channel == message.channel_id {
                continue;
            }
            if let Err(message) =
                channel.send_message(&context.http, |m| m.content(&content).files(files.clone()))
            {
                error!("unable to mirror message to discord: {:?}", message);
            }
        }
    }
}

use crate::notifiers::BaseClient;
use async_trait::async_trait;
use log::error;

use serenity::{
    prelude::{EventHandler, GatewayIntents},
    Client,
};
use std::env;

use super::Discordable;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}
pub struct BycDiscordClient {}

impl BycDiscordClient {
    pub async fn get_server() -> Result<Client, serenity::Error> {
        let token = env::var("DISCORD_TOKEN").expect("discord token is not set");
        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
        Client::builder(token, intents).await
    }
}

#[async_trait(?Send)]
impl<T: Discordable> BaseClient<T> for BycDiscordClient {
    async fn send_message(&self, messageable: &T) {
        let channel = messageable.get_channel();
        if !channel.sendable {
            return;
        }
        let msg = messageable.to_message().await;
        let client = match BycDiscordClient::get_server().await {
            Ok(r) => r,
            _ => return,
        };
        if msg.is_none() {
            return;
        }
        for webhook in channel.webhooks {
            let webhook_message = match msg.as_ref().unwrap().as_object() {
                Some(x) => x.to_owned(),
                None => continue,
            };
            if !(webhook.send_condition.unwrap())(webhook_message.clone()) {
                continue;
            }
            if let Err(error) = client
                .cache_and_http
                .http
                .execute_webhook(webhook.id, &webhook.token, false, &webhook_message)
                .await
            {
                error!("{:?}", error);
                drop(webhook_message);
                drop(error);
            };
        }
        if channel.id == 0 {
            return;
        }

        match client
            .cache_and_http
            .http
            .send_message(channel.id, &msg.unwrap())
            .await
        {
            Ok(_res) => {}
            Err(err) => {
                error!("{:?}", err);

                drop(err)
            }
        };
    }
}

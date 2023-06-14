use super::{discord::BycDiscordClient, webhook::client::WebhookClient};

#[derive(Clone, Copy, Default)]
pub struct Notifier<'a> {
    pub discord: Option<&'a BycDiscordClient>,
    pub webhooks: Option<&'a Vec<WebhookClient>>,
}

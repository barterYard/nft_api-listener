use super::webhook::client::WebhookClient;
use flow_helpers::mongo::mongodb::Client;

#[derive(Clone, Copy, Default)]
pub struct Notifier<'a> {
    pub webhooks: Option<&'a Vec<WebhookClient>>,
    pub database: Option<&'a Client>,
}

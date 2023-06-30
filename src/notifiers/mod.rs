use async_trait::async_trait;

mod notifier;
pub mod webhook;

use crate::listeners::Messageable;
pub use notifier::Notifier;

#[async_trait(?Send)]
pub trait BaseClient<T>
where
    T: Messageable,
{
    async fn send_message(&self, messageable: &T);
}

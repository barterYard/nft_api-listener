mod channel;
mod client;

use crate::listeners::Messageable;
pub use channel::Channel;
pub use client::BycDiscordClient;

pub trait Discordable: Messageable {
    fn get_channel(&self) -> Channel;
}

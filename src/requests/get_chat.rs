use async_trait::async_trait;

use crate::bot::Bot;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{Chat, ChatId},
};

/// Use this method to get up to date information about the chat
/// (current name of the user for one-on-one conversations,
/// current username of a user, group or channel, etc.).
/// Returns a Chat object on success.
#[derive(Debug, Clone, Serialize)]
pub struct GetChat<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username
    /// of the target supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
}

#[async_trait]
impl Request for GetChat<'_> {
    type Output = Chat;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl GetChat<'_> {
    pub async fn send(self) -> ResponseResult<Chat> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getChat",
            &self,
        )
        .await
    }
}

impl<'a> GetChat<'a> {
    pub fn chat_id<C>(mut self, value: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }
}

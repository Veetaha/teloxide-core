// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{InlineKeyboardMarkup, LabeledPrice, Message};

impl_payload! {
    /// Use this method to send invoices. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendInvoice (SendInvoiceSetters) => Message {
        required {
            /// Unique identifier for the target private chat
            pub chat_id: i32,
            /// Product name, 1-32 characters
            pub title: String [into],
            /// Product description, 1-255 characters
            pub description: String [into],
            /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
            pub payload: String [into],
            /// Payments provider token, obtained via [Botfather]
            ///
            /// [Botfather]: https://t.me/botfather
            pub provider_token: String [into],
            /// Unique deep-linking parameter that can be used to generate this invoice when used as a start parameter
            pub start_parameter: String [into],
            /// Three-letter ISO 4217 currency code, see more on currencies
            pub currency: String [into],
            /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
            pub prices: Vec<LabeledPrice> [collect],
        }
        optional {
            /// A JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
            pub provider_data: String [into],
            /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
            pub photo_url: String [into],
            /// Photo size
            pub photo_size: String [into],
            /// Photo width
            pub photo_width: String [into],
            /// Photo height
            pub photo_height: String [into],
            /// Pass _True_, if you require the user's full name to complete the order
            pub need_name: bool,
            /// Pass _True_, if you require the user's phone number to complete the order
            pub need_phone_number: bool,
            /// Pass _True_, if you require the user's email address to complete the order
            pub need_email: bool,
            /// Pass _True_, if you require the user's shipping address to complete the order
            pub need_shipping_address: bool,
            /// Pass _True_, if user's phone number should be sent to provider
            pub send_phone_number_to_provider: bool,
            /// Pass _True_, if user's email address should be sent to provider
            pub send_email_to_provider: bool,
            /// Pass _True_, if the final price depends on the shipping method
            pub is_flexible: bool,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// If the message is a reply, ID of the original message
            pub reply_to_message_id: i32,
            /// Pass _True_, if the message should be sent even if the specified replied-to message is not found
            pub allow_sending_without_reply: bool,
            /// A JSON-serialized object for an [inline keyboard]. If empty, one 'Pay `total price`' button will be shown. If not empty, the first button must be a Pay button.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: InlineKeyboardMarkup,
        }
    }
}
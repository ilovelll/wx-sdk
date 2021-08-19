use std::time::Duration;

use crate::{
    access_token::AccessTokenProvider,
    error::{CommonResponse, SdkError, SdkResult},
    wechat::WxApiRequestBuilder,
    WxSdk,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Ticket {
    pub ticket: String,
    pub expires_in: i32,
}

pub enum TicketType {
    JsApi,
    WxCard,
}

impl std::fmt::Display for TicketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketType::JsApi => write!(f, "jsapi"),
            TicketType::WxCard => write!(f, "wx_card"),
        }
    }
}

pub struct TicketModule<'a, T: AccessTokenProvider>(pub(crate) &'a WxSdk<T>);
impl<'a, T: AccessTokenProvider> TicketModule<'a, T> {
    pub async fn get_ticket(&self, t_type: TicketType) -> SdkResult<Ticket> {
        let cache_ticket = self.0.cache.get(&t_type.to_string()).await;
        if let Some(ticket) = cache_ticket {
            return Ok(Ticket {
                ticket,
                expires_in: 0,
            });
        } else {
            let base_url = "https://api.weixin.qq.com/cgi-bin/ticket/getticket";
            let sdk = self.0;
            let builder = sdk.wx_get(base_url).await?;

            let builder = builder.query(&[("type", t_type.to_string())]);
            let res: CommonResponse<Ticket> = builder.send().await?.json().await?;
            if let CommonResponse::Ok(t) = &res {
                self.0
                    .cache
                    .set(
                        t_type.to_string(),
                        t.ticket.clone(),
                        Some(Duration::from_secs((t.expires_in - 5) as u64)),
                    )
                    .await;
            }
            res.into()
        }
    }
}

#[cfg(test)]
mod tests {

    use tokio::time::sleep;

    use crate::{
        mp::ticket::TicketType,
        wechat::{EncodingMode, ServerConfig},
        TokenClient, WxSdk,
    };

    fn get_sdk() -> WxSdk<TokenClient> {
        let server_config = ServerConfig::new(
            "QDG6eK",
            EncodingMode::Security("jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C".to_owned()),
        );
        let wsdk =
            WxSdk::new_with_default_token_client("wx5823bf96d3bd56c7", "app_secret", server_config);
        wsdk
    }

    #[tokio::test]
    async fn test_get_from_cache() {
        use std::time::Duration;
        let sdk = get_sdk();
        let mp_sdk = sdk.mp();
        assert_eq!(TicketType::JsApi.to_string(), "jsapi");
        assert_eq!(TicketType::WxCard.to_string(), "wx_card");
        let t_type = TicketType::JsApi;
        mp_sdk
            .0
            .cache
            .set(
                t_type.to_string(),
                "ticket_value".to_owned(),
                Some(Duration::from_secs((7200 - 5) as u64)),
            )
            .await;
        sleep(Duration::from_secs(2)).await;
        let ticket = mp_sdk.ticket().get_ticket(TicketType::JsApi).await.unwrap();
        assert_eq!(ticket.ticket, "ticket_value");
        assert_eq!(ticket.expires_in, 0);
    }
}

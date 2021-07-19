pub mod access_token;
use std::{pin::Pin, task::{Context, Poll}};

pub use access_token::AccessToken;

pub mod error;
pub mod office_account;
pub use error::SdkResult;
pub mod wechat;
pub use access_token::TokenClient;
use futures::Future;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct MyFuture {}

impl Future for MyFuture{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        
        todo!()
    }
}
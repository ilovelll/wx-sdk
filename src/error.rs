//! This module define the types for error handling.
//!
//! Most the function call on `wx-sdk` return a [SdkResult], it's a type of `std::result::Result<T, SdkError>` wrapper.

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;
// use tonic::codegen::http::request;

/// Almost every WeChat's api calling will return a JSON value contains `errcode` and `errmsg`, that is a struct for it.
/// When the `errcode == 0`,  it can transmute to `SdkResult<()>`.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Error)]
#[error("Wechat API response error: errcode {errcode}, errmsg {errmsg}")]
pub struct CommonError {
    pub errcode: i32,
    pub errmsg: String,
}

/// Enum for the return result of http calling WeChat's api.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum CommonResponse<T> {
    Ok(T),
    Err(CommonError),
}

/// The SDK self-defined error enum.
#[derive(Error, Debug)]
pub enum SdkError {
    #[error("reqwest Error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("wx received event parse error")]
    XmlParseError(#[from] roxmltree::Error),
    #[error("get access token error")]
    AccessTokenError(CommonError),
    #[error(transparent)]
    WxApiError(#[from] CommonError),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error("api request params error: {0}")]
    InvalidParams(String),
    #[error("parse received encrypt msg error: wrong signature")]
    InvalidSignature,
    #[error("parse received encrypt msg error: invalid appid")]
    InvalidAppid,
    #[error("decrypt msg error: {0}")]
    MsgDecryptError(String),
    #[error("encrypt msg error: {0}")]
    MsgEncryptError(String),
}

/// A wrap of `std::result::Result<T, SdkError>`.
pub type SdkResult<T> = std::result::Result<T, SdkError>;

/// When the `errcode == 0`,  it can transmute to `SdkResult<()>`.
impl From<CommonError> for SdkResult<()> {
    fn from(e: CommonError) -> Self {
        if e.errcode == 0 {
            Ok(())
        } else {
            Err(SdkError::WxApiError(e))
        }
    }
}

/// Trans CommonResponse<T> to SdkResult<T> when the error case is `SdkError::WxApiError(_)`.
impl<T> From<CommonResponse<T>> for SdkResult<T> {
    fn from(r: CommonResponse<T>) -> Self {
        match r {
            CommonResponse::Ok(t) => Ok(t),
            CommonResponse::Err(e) => Err(SdkError::WxApiError(e)),
        }
    }
}

/// Unwrap the `CommonResponse<CommonError>` to SdkResult<()> or `SdkError::WxApiError(_)`.
impl From<CommonResponse<CommonError>> for SdkResult<()> {
    fn from(r: CommonResponse<CommonError>) -> Self {
        match r {
            CommonResponse::Ok(e) => {
                if e.errcode == 0 {
                    Ok(())
                } else {
                    Err(SdkError::WxApiError(e))
                }
            }
            CommonResponse::Err(e) => {
                if e.errcode == 0 {
                    Ok(())
                } else {
                    Err(SdkError::WxApiError(e))
                }
            }
        }
    }
}

#[test]
fn test_error_from() {
    let input = r#"{"errcode": 0,"errmsg":"success"}"#;
    let expected = CommonResponse::Ok(CommonError {
        errcode: 0,
        errmsg: "success".to_string(),
    });
    assert_eq!(expected.clone(), serde_json::from_str(input).unwrap());

    let into: SdkResult<()> = expected.clone().into();
    assert!(into.is_ok());

    let input = r#"{"errcode":40013,"errmsg":"invalid appid"}"#;
    let expected = CommonResponse::Ok(CommonError {
        errcode: 40013,
        errmsg: "invalid appid".to_string(),
    });
    assert_eq!(expected.clone(), serde_json::from_str(input).unwrap());

    let into: SdkResult<()> = expected.clone().into();
    assert!(into.is_err());
}

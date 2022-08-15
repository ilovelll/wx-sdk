use std::path::Path;

#[derive(Clone)]
pub struct PaySdk {
    pub(crate) config: PaySdkConfig,
}

#[derive(Clone)]
pub struct PaySdkConfig {
    mch_id: String,
    mch_certificate_serial_number: String,
    mch_apiv3_key: String,
    mch_private_key: String,
}

impl PaySdkConfig {
    pub fn new<S: Into<String>>(
        mch_id: S,
        mch_certificate_serial_number: S,
        mch_apiv3_key: S,
        mch_private_path: S,
    ) {
        todo!()
    }
}

// pub(crate) mod utils;

use aes::Aes256;
use base64ct::{Base64, Encoding};
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

use crate::error::SdkError;
use crate::SdkResult;

// create an alias for convenience
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn decrypt_message<'a, S: AsRef<str>>(raw: &'a str, key: S) -> SdkResult<Vec<u8>> {
    let mut key_buf = [0u8; 32];
    let mut iv_buf = [0u8; 16];
    // first: base64 decode the key
    let key = Base64::decode(format!("{}=", key.as_ref()), &mut key_buf)
        .map_err(|e| SdkError::MsgDecryptError(e.to_string()))?;
    iv_buf.copy_from_slice(&key[0..16]);
    // new the cipher
    let cipher = Aes256Cbc::new_from_slices(&key, &iv_buf)
        .map_err(|e| SdkError::MsgDecryptError(e.to_string()))?;

    let mut dec_buf = [0u8; 4096];
    // second: base65 decode the raw message
    let encrypt_buf =
        Base64::decode(raw, &mut dec_buf).map_err(|e| SdkError::MsgDecryptError(e.to_string()))?;
    let mut encrypt_buf = encrypt_buf.to_owned();
    // last: decrypt the message
    let decrypted_ciphertext = cipher
        .decrypt(&mut encrypt_buf)
        .map_err(|e| SdkError::MsgDecryptError(e.to_string()))?;
    Ok(decrypted_ciphertext.to_owned())
}

pub fn encrypt_message() {
    todo!()
}

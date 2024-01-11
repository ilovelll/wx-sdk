use crate::utils::{base64, get_random_string};
use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

use crate::error::SdkError;
use crate::SdkResult;

// create an alias for convenience
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

/// 解密消息，返回(decrypted_msg, app_id)
pub fn decrypt_message<'a, S: AsRef<str>>(
    ciphertext: &'a str,
    key: S,
) -> SdkResult<(String, String)> {
    // let mut key_buf = [0u8; 32];
    let mut iv_buf = [0u8; 16];
    // first: base64 decode the key
    let key = base64::decode(format!("{}=", key.as_ref()))
        .map_err(|e| SdkError::MsgDecryptError(e.to_string()))?;
    iv_buf.copy_from_slice(&key[0..16]);
    // new the cipher
    let cipher = Aes256Cbc::new_from_slices(&key[0..32], &iv_buf)
        .map_err(|e| SdkError::MsgDecryptError(e.to_string()))?;

    // second: base65 decode the raw message
    let mut encrypt_buf =
        base64::decode(ciphertext).map_err(|e| SdkError::MsgDecryptError(e.to_string()))?;

    // last: decrypt the message
    let decrypted_ciphertext = cipher
        .decrypt_vec(&mut encrypt_buf)
        .map_err(|e| SdkError::MsgDecryptError(e.to_string()))?;

    // decrpyted_text = [random(16) + content_len(4) + content + appid]
    let (_, text) = decrypted_ciphertext.split_at(16);
    let (xlen, text) = text.split_at(4);
    let mut len = [0; 4];
    len.copy_from_slice(&xlen[..]);
    let len = u32::from_be_bytes(len);
    let (text, appid) = text.split_at(len as usize);

    Ok((
        String::from_utf8_lossy(text).to_string(),
        String::from_utf8_lossy(appid).to_string(),
    ))
}

pub fn encrypt_message<'a, S: AsRef<str>>(
    plaintext: &'a str,
    key: S,
    app_id: S,
) -> SdkResult<String> {
    let plaintext = plaintext.as_bytes();
    // let mut key_buf = [0u8; 32];
    let mut iv_buf = [0u8; 16];
    // first: base64 decode the key
    let key = base64::decode(format!("{}=", key.as_ref()))
        .map_err(|e| SdkError::MsgEncryptError(e.to_string()))?;
    iv_buf.copy_from_slice(&key[0..16]);
    // new the cipher
    let cipher = Aes256Cbc::new_from_slices(&key[0..32], &iv_buf)
        .map_err(|e| SdkError::MsgEncryptError(e.to_string()))?;

    // encrpyted_text = [random(16) + content_len(4) + content + appid]
    let random_text = get_random_string(16);
    let mut buf = random_text.into_bytes();

    let len = plaintext.len() as u32;
    let len = len.to_be_bytes();
    buf.extend_from_slice(&len);

    buf.extend_from_slice(plaintext);
    let app_id = app_id.as_ref().as_bytes();
    buf.extend_from_slice(app_id);

    // encrypt the message
    let ciphertext = cipher.encrypt_vec(buf.as_slice());

    // last: base65 encode the raw message
    let ciphertext = base64::encode(ciphertext.as_slice());
    Ok(ciphertext)
}

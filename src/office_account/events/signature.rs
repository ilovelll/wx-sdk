use serde_derive::Deserialize;
use sha1::{Digest, Sha1};

#[derive(Debug, Deserialize)]
pub struct Signature {
    signature: String,
    data: Vec<String>,
}

impl Signature {
    pub fn new<S: AsRef<str>>(signature: S, input: Vec<String>) -> Self {
        Signature {
            signature: signature.as_ref().to_owned(),
            data: input
        }
    }
	pub fn is_ok(&self) -> bool {
		let mut arr = self.data.clone();
		arr.sort();
		let str = arr.join("");

		let mut hasher = Sha1::new();
		hasher.update(str);
		let result = hasher.finalize();
		format!("{:x}", result) == self.signature
	}
}

use cloudfront_sign::{get_signed_url, SignedOptions};
use lazy_static::lazy_static;
use std::fs;

lazy_static! {
    static ref AWS_CLOUDFRONT_PRIVATE_KEY: String = fs::read_to_string(
        std::env::var("AWS_CLOUDFRONT_PRIVATE_KEY").expect("Can't read AWS_CLOUDFRONT_PRIVATE_KEY")
    )
    .expect("Can't read file");
    static ref AWS_CLOUDFRONT_KEY_PAIR_ID: String =
        std::env::var("AWS_CLOUDFRONT_KEY_PAIR_ID").expect("Can't read AWS_CLOUDFRONT_KEY_PAIR_ID");
    static ref AWS_CLOUDFRONT_URL: String =
        std::env::var("AWS_CLOUDFRONT_URL").expect("Can't read AWS_CLOUDFRONT_URL");
}

pub struct CloudFrontSigner;

impl CloudFrontSigner {
    pub fn sing(key: Option<String>) -> Option<String> {
        let options = SignedOptions {
            key_pair_id: AWS_CLOUDFRONT_KEY_PAIR_ID.clone(),
            private_key: AWS_CLOUDFRONT_PRIVATE_KEY.clone(),
            ..Default::default()
        };

        if let Some(file_key) = key {
            return get_signed_url(
                &format!("{}/{}", AWS_CLOUDFRONT_URL.clone(), file_key),
                &options,
            )
            .ok();
        }

        None
    }
}

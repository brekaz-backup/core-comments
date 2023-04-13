use blumer_lib_authorization_rs::clients::post::Post;
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use blumer_lib_errors::AppError;
use cloudfront_sign::{get_signed_url, SignedOptions};
use uuid::Uuid;

pub fn comment_description_max_len(comment: &str) -> bool {
    const MAX_LEN: usize = 200;
    comment.chars().count() > MAX_LEN
}

pub async fn s3_get_signed_url(
    aws_cloudfront_url: &String,
    key_pair_id: &String,
    private_key: &String,
    key: Option<String>,
) -> Option<String> {
    let signed_options = SignedOptions {
        key_pair_id: key_pair_id.to_string(),
        private_key: private_key.to_string(),
        ..Default::default()
    };
    if let Some(key) = key {
        let url: &str = &format!("{}/{}", aws_cloudfront_url, &key[..]);
        let signed_url = get_signed_url(&url, &signed_options)
            .expect("Error when getting CloudFront signed url");
        Some(signed_url)
    } else {
        None
    }
}
pub async fn can_view_post(
    mut post_client: PostAuthorization,
    post_id: Uuid,
    user_id: Uuid,
) -> Result<Post, AppError> {
    let can_view_post: Post = post_client
        .can_view_post(post_id, user_id)
        .await
        .map_err(|e| {
            log::error!("Error when trying to get can_view_post: {}", e);
            AppError::ServerError(format!("Error when trying to get can_view_post: {}", e))
        })?;

    if !can_view_post.valid {
        return Err(AppError::ValidationError {
            reason: can_view_post.reason.to_string(),
            code: can_view_post.reason,
        });
    }
    Ok(can_view_post)
}

use blumer_lib_authorization_rs::clients::post::Post;
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use blumer_lib_errors::AppError;
use cloudfront_sign::{get_signed_url, SignedOptions};
use uuid::Uuid;

pub fn comment_description_max_len(comment: &str) -> bool {
    const MAX_LEN: usize = 200;
    comment.chars().count() > MAX_LEN
}

pub async fn s3_get_signed_url(key: Option<String>) -> Option<String> {
    let private_key = tokio::fs::read_to_string("private_key.pem")
        .await
        .expect("Error when reading private key");
    let options = SignedOptions {
        key_pair_id: String::from("K3TWLQ3HHETAG3"),
        private_key: private_key,
        ..Default::default()
    };
    if let Some(key) = key {
        let url: &str = &format!("https://d1pcqt6t6kr36e.cloudfront.net/{}", &key[..]);
        let signed_url =
            get_signed_url(&url, &options).expect("Error when getting CloudFront signed url");
        // println!("{}", &signed_url);
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

/*pub async fn can_view_comment(
    ctx: &Context<'_>,
    post_id: Uuid,
    user_id: Uuid,
)-> Result<Comment, AppError> {
    Ok(Comment{})
}*/

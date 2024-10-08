use blumer_lib_authorization_rs::clients::post::Post;
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use blumer_lib_errors::AppError;
use uuid::Uuid;

pub fn comment_description_contains_forbidden_words(comment: &str) -> bool {
    let blacklist_comments = ["http://", "https://", "www.", ".com", ".app", ".link"];
    blacklist_comments
        .iter()
        .any(|&element| comment.contains(element))
}

pub fn comment_description_max_len(comment: &str) -> bool {
    const MAX_LEN: usize = 250;
    comment.chars().count() > MAX_LEN
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
            reason: can_view_post.reason,
            code: "CAN_NOT_SEE_POST".to_owned(),
        });
    }
    Ok(can_view_post)
}

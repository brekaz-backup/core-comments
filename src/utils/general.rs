use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use blumer_lib_authorization_rs::clients::post::{Post, PostData};
use blumer_lib_errors::AppError;
use uuid::Uuid;

pub fn comment_description_max_len(comment: &str) -> bool {
    const MAX_LEN: usize = 250;
    comment.chars().count() > MAX_LEN
}

pub async fn can_view_post(
    mut _post_client: PostAuthorization,
    post_id: Uuid,
    _user_id: Uuid,
) -> Result<Post, AppError> {
    /*let can_view_post: Post = post_client
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
    }*/
    Ok(Post {
        valid: true,
        reason: "ok".to_owned(),
        data: Some(PostData {
            post_id: post_id,
            owner_id: Uuid::default(),
        }),
    })
}

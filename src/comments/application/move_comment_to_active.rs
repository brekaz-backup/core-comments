use crate::comments::domain::{CommentCreateEntity, CommentRepositoryInterface};
use crate::media_comments::infrastructure::capn_proto::models::{
    CommentUploadEntity, FileTypeEntity,
};
use anyhow::Result;
use blumer_lib_errors::AppError;
use std::str::FromStr;
use uuid::Uuid;

pub struct MoveCommentToActiveUseCase;

impl MoveCommentToActiveUseCase {
    pub async fn execute(
        comment_repo: impl CommentRepositoryInterface,
        comment: CommentUploadEntity,
    ) -> Result<(), AppError> {
        //find inactive comment
        let comment_db = comment_repo
            .get_inactive_comment_by_id(
                &Uuid::from_str(&comment.parent_id)
                    .map_err(|e| AppError::DatasourceError(e.to_string()))?,
                &Uuid::from_str(&comment.comment_id)
                    .map_err(|e| AppError::DatasourceError(e.to_string()))?,
            )
            .await?
            .ok_or(AppError::DatasourceError(
                "Error getting comment".to_owned(),
            ))?;

        //save comment
        comment_repo
            .create_comment(CommentCreateEntity {
                post_id: comment_db.post_id,
                user_id: comment_db.user_id,
                comment_id: comment_db.comment_id,
                description: comment_db.description,
                image: match comment.file_type {
                    FileTypeEntity::Image => Some(comment.file_key.to_string()),
                    _ => None,
                },
                audio: match comment.file_type {
                    FileTypeEntity::Audio => Some(comment.file_key),
                    _ => None,
                },
                gif: comment_db.gif,
                active: true,
            })
            .await?;

        //delete inactive comment
        comment_repo
            .delete_inactive_comment(&comment_db.post_id, &comment_db.comment_id)
            .await?;

        Ok(())
    }
}

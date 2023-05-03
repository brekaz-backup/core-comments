use crate::media_comments::infrastructure::capn_proto::models::{
    CommentUploadEntity, FileTypeEntity,
};
use crate::reply_comments::domain::{
    CommentReplyCreateEntity, CommentReplyDeleteEntity, ReplyCommentRepositoryInterface,
};
use anyhow::Result;
use blumer_lib_errors::AppError;
use std::str::FromStr;
use uuid::Uuid;

pub struct MoveReplyCommentToActiveUseCase;

impl MoveReplyCommentToActiveUseCase {
    pub async fn execute(
        reply_comment_repo: impl ReplyCommentRepositoryInterface,
        reply_comment: CommentUploadEntity,
    ) -> Result<(), AppError> {
        //find inactive reply comment
        let reply_comment_db = reply_comment_repo
            .get_inactive_reply_comment_by_id(
                &Uuid::from_str(&reply_comment.parent_id)
                    .map_err(|e| AppError::DatasourceError(e.to_string()))?,
                &Uuid::from_str(&reply_comment.comment_id)
                    .map_err(|e| AppError::DatasourceError(e.to_string()))?,
                &Uuid::from_str(
                    &reply_comment
                        .reply_id
                        .ok_or(AppError::DatasourceError("reply_id null".to_owned()))?,
                )
                .map_err(|e| AppError::DatasourceError(e.to_string()))?,
            )
            .await?
            .ok_or(AppError::DatasourceError(
                "Error getting comment".to_owned(),
            ))?;

        //save reply comment
        reply_comment_repo
            .create_comment_reply(CommentReplyCreateEntity {
                post_id: reply_comment_db.post_id,
                reply_id: reply_comment_db.reply_id,
                user_id: reply_comment_db.user_id,
                comment_id: reply_comment_db.comment_id,
                description: reply_comment_db.description,
                image: match reply_comment.file_type {
                    FileTypeEntity::Image => Some(reply_comment.file_key.to_string()),
                    _ => None,
                },
                audio: match reply_comment.file_type {
                    FileTypeEntity::Audio => Some(reply_comment.file_key),
                    _ => None,
                },
                gif: reply_comment_db.gif,
                active: true,
            })
            .await?;

        //delete inactive reply comment
        reply_comment_repo
            .delete_inactive_reply_comment(CommentReplyDeleteEntity {
                post_id: reply_comment_db.post_id,
                comment_id: reply_comment_db.comment_id,
                reply_id: reply_comment_db.reply_id,
            })
            .await?;

        Ok(())
    }
}

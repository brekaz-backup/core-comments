use super::objects::CommentReplyOutput;
use crate::reply_comments::domain::CommentReplyEntity;
use anyhow::Result;
use blumer_lib_errors::AppError;
use chrono::{DateTime, Duration, Local, TimeZone};

fn get_datetime_from_scylla(duration: Duration) -> Result<String, AppError> {
    let initial_datetime: DateTime<Local> = match Local.with_ymd_and_hms(1970, 1, 1, 0, 0, 0) {
        chrono::LocalResult::None => {
            return Err(AppError::DatasourceError(
                "Error when converting scylla datetime to string".to_owned(),
            ));
        }
        chrono::LocalResult::Single(d) => d,
        chrono::LocalResult::Ambiguous(d1, _d2) => d1,
    };

    let final_datetime =
        initial_datetime
            .checked_add_signed(duration)
            .ok_or(AppError::DatasourceError(
                "Error when converting scylla datetime to string".to_owned(),
            ))?;
    Ok(final_datetime.format("%Y-%m-%dT%H:%M:%S.%3f%Z").to_string())
}

pub struct ReplyCommentMapper;

impl ReplyCommentMapper {
    pub fn object(entity: CommentReplyEntity) -> Result<CommentReplyOutput> {
        Ok(CommentReplyOutput {
            id: entity.reply_id.to_string().into(),
            post_id: entity.post_id,
            comment_id: entity.comment_id,
            user_id: entity.user_id,
            description: entity.description,
            image: entity.image,
            audio: entity.audio,
            gif: entity.gif,
            created_at: get_datetime_from_scylla(entity.created_at).map_err(|_| {
                AppError::ServerError("Error when converting scylla datetime to string".to_string())
            })?,
        })
    }
}

use super::objects::CommentReplyOutput;
use crate::reply_comments::domain::CommentReplyEntity;
use chrono::{Local, TimeZone};

pub struct ReplyCommentMapper;

impl ReplyCommentMapper {
    pub fn object(entity: CommentReplyEntity) -> CommentReplyOutput {
        let initial_datetime = Local.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
        let final_datetime = initial_datetime
            .checked_add_signed(entity.created_at)
            .unwrap();
        let created_at = final_datetime.format("%Y-%m-%dT%H:%M:%S.%3f%Z").to_string();
        CommentReplyOutput {
            id: entity.reply_id.to_string().into(),
            post_id: entity.post_id,
            comment_id: entity.comment_id,
            user_id: entity.user_id,
            description: entity.description,
            image: entity.image,
            audio: entity.audio,
            gif: entity.gif,
            created_at,
        }
    }
}

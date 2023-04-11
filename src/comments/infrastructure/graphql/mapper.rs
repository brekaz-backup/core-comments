use super::objects::CommentOutput;
use crate::comments::domain::CommentEntity;
use chrono::{Local, TimeZone};

pub struct CommentMapper;

impl CommentMapper {
    pub fn object(entity: CommentEntity) -> CommentOutput {
        let initial_datetime = Local.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
        let final_datetime = initial_datetime
            .checked_add_signed(entity.created_at)
            .unwrap();
        let created_at = final_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        CommentOutput {
            id: entity.comment_id.to_string().into(),
            post_id: entity.post_id,
            user_id: entity.user_id,
            description: entity.description,
            image: entity.image,
            audio: entity.audio,
            gif: entity.gif,
            created_at,
        }
    }

    /*pub fn entity(object: CommentObject) -> CommentEntity {
        CommentEntity {
            comment_id: Uuid::parse_str(&object.id).unwrap(),
            post_id: object.post_id,
            user_id: object.user_id,
            description: object.description,
            image: object.image,
            audio: object.audio,
            gif: object.gif,
            created_at: Duration::::parse(),
        }
    }*/
}

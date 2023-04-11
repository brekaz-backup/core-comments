use chrono::Duration;
use uuid::Uuid;

#[derive(Clone, scylla::FromRow)]
pub struct CommentReplyEntity {
    pub post_id: Uuid,
    pub reply_id: Uuid,
    pub comment_id: Uuid,
    pub user_id: Uuid,
    pub description: Option<String>,
    pub image: Option<String>,
    pub audio: Option<String>,
    pub gif: Option<String>,
    pub created_at: Duration,
}

#[derive(Clone)]
pub struct CommentReplyCreateEntity {
    pub post_id: Uuid,
    pub reply_id: Uuid,
    pub user_id: Uuid,
    pub comment_id: Uuid,
    pub description: Option<String>,
    pub image: Option<String>,
    pub audio: Option<String>,
    pub gif: Option<String>,
    pub active: bool,
}

#[derive(Clone)]
pub struct CommentReplyDeleteEntity {
    pub post_id: Uuid,
    pub comment_id: Uuid,
    pub reply_id: Uuid,
}

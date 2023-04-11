use chrono::Duration;
use uuid::Uuid;

#[derive(Clone, scylla::FromRow)]
pub struct CommentEntity {
    pub comment_id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub description: Option<String>,
    pub image: Option<String>,
    pub audio: Option<String>,
    pub gif: Option<String>,
    pub created_at: Duration,
}

#[derive(Clone)]
pub struct CommentCreateEntity {
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub comment_id: Uuid,
    pub description: Option<String>,
    pub image: Option<String>,
    pub audio: Option<String>,
    pub gif: Option<String>,
    pub active: bool,
}

#[derive(Clone)]
pub struct CommentDeleteEntity {
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub post_owner_id: Uuid,
    pub comment_id: Uuid,
}

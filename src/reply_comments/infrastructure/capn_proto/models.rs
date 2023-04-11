use uuid::Uuid;

#[derive(Clone)]
pub struct CommentReplyCreateEntity {
    pub post_id: Uuid,
    pub reply_id: Uuid,
    pub user_id: Uuid,
    pub comment_id: Uuid,
    pub comment_owner_id: Uuid,
    pub description: Option<String>,
    pub image: Option<String>,
    pub audio: Option<String>,
    pub gif: Option<String>,
    pub active: bool,
    pub post_owner_id: Uuid,
}

#[derive(Clone)]
pub struct ReplyCountsEntity {
    pub comment_id: Uuid,
    pub reply_id: Uuid,
    pub reactions_count_1: i16,
    pub reactions_count_2: i16,
    pub reactions_count_3: i16,
    pub reactions_count_4: i16,
    pub reactions_count_5: i16,
    pub reactions_count_6: i16,
}

#[derive(Clone)]
pub struct CommentRepliesCounterEntity {
    pub comment_id: Uuid,
    pub replies_count: i16,
}

#[derive(Clone)]
pub struct CommentReplyDeleteEntity {
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub comment_id: Uuid,
    pub reply_id: Uuid,
    pub post_owner_id: Uuid,
}

#[derive(Clone)]
pub struct ReplyCountsDeleteEntity {
    pub comment_id: Uuid,
    pub reply_id: Uuid,
}

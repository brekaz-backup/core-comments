use crate::comments::infrastructure::graphql::objects::CommentTypeInput;
use async_graphql::*;
use uuid::Uuid;

#[derive(InputObject)]
pub struct CommentsReplyCreateInput {
    pub post_id: ID,
    pub comment_id: ID,
    pub description: Option<String>,
    pub gif: Option<String>,
    pub comment_type: CommentTypeInput,
}

#[derive(InputObject, Debug)]
pub struct CommentsReplyDeleteInput {
    pub post_id: ID,
    pub comment_id: ID,
    pub reply_id: ID,
}

#[derive(SimpleObject, Default, Clone)]
#[graphql(complex)]
pub struct CommentReplyOutput {
    pub id: ID,
    #[graphql(shareable)]
    pub post_id: Uuid,
    pub comment_id: Uuid,
    pub user_id: Uuid,
    pub description: Option<String>,
    pub image: Option<String>,
    pub audio: Option<String>,
    pub gif: Option<String>,
    pub created_at: String,
}

#[derive(SimpleObject, Clone, Default)]
pub struct ReplyResponseOutput {
    pub comments: Vec<CommentReplyOutput>,
    pub next_page: Option<String>,
}

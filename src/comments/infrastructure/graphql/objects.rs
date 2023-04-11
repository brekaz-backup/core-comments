use async_graphql::*;
use uuid::Uuid;

#[derive(SimpleObject)]
pub struct SimpleProfile {
    pub id: ID,
}

#[derive(SimpleObject, Default, Clone)]
#[graphql(complex)]
pub struct CommentOutput {
    pub id: ID,
    #[graphql(shareable)]
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub description: Option<String>,
    pub image: Option<String>,
    pub audio: Option<String>,
    pub gif: Option<String>,
    pub created_at: String,
}

#[derive(SimpleObject, Clone, Default)]
pub struct CommentResponseOutput {
    pub comments: Vec<CommentOutput>,
    pub next_page: Option<String>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum CommentTypeInput {
    Text,
    TextAndImage,
    Image,
    Audio,
    Gif,
}

#[derive(InputObject)]
pub struct CommentsCreateInput {
    pub post_id: ID,
    pub description: Option<String>,
    pub gif: Option<String>,
    pub comment_type: CommentTypeInput,
}

#[derive(InputObject, Debug)]
pub struct CommentsDeleteInput {
    pub post_id: ID,
    pub comment_id: ID,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Created,
    Error,
}

#[derive(SimpleObject)]
pub struct CreateOutput {
    pub status: Status,
    pub message: String,
}

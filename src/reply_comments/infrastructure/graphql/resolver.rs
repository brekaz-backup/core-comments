use super::objects::CommentReplyOutput;
use crate::comments::infrastructure::graphql::objects::SimpleProfile;
use async_graphql::*;

#[ComplexObject]
impl CommentReplyOutput {
    async fn profile(&self) -> SimpleProfile {
        SimpleProfile {
            id: self.user_id.clone().into(),
        }
    }
}

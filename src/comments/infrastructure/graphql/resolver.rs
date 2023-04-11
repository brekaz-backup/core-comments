use super::objects::{CommentOutput, SimpleProfile};
use async_graphql::*;

#[ComplexObject]
impl CommentOutput {
    async fn profile(&self) -> SimpleProfile {
        SimpleProfile {
            id: self.user_id.clone().into(),
        }
    }
}

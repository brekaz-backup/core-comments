pub mod config;
pub mod state;
use crate::comments::infrastructure::graphql::{mutation::CommentMutation, query::CommentQuery};
use crate::reply_comments::infrastructure::graphql::{
    mutation::ReplyCommentMutation, query::ReplyCommentQuery,
};
use async_graphql::{EmptySubscription, MergedObject, Schema};

#[derive(MergedObject, Default)]
pub struct Query(CommentQuery, ReplyCommentQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(CommentMutation, ReplyCommentMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

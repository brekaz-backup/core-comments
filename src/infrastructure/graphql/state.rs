use crate::adapters::kafka::KafkaProducer;
use crate::comments::domain::{CommentRedisRepository, CommentRepository};
use crate::reply_comments::domain::{ReplyCommentRedisRepository, ReplyCommentRepository};

#[derive(Clone)]
pub struct AppState {
    pub comment_repository: CommentRepository,
    pub comment_redis_repository: CommentRedisRepository,
    pub reply_comment_repository: ReplyCommentRepository,
    pub reply_comment_redis_repository: ReplyCommentRedisRepository,
    pub kafka_producer: KafkaProducer,
}

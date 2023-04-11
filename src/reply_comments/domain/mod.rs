mod reply_comment;
mod reply_comment_redis_repository;
mod reply_comment_redis_repository_interface;
mod reply_comment_repository;
mod reply_comment_repository_interface;

pub use {
    reply_comment::*, reply_comment_redis_repository::ReplyCommentRedisRepository,
    reply_comment_redis_repository_interface::ReplyCommentRedisRepositoryInterface,
    reply_comment_repository::ReplyCommentRepository,
    reply_comment_repository_interface::ReplyCommentRepositoryInterface,
};

pub static CREATE_COMMENT_REPLY: &str = r#"
    INSERT INTO comments.comments_reply (post_id, reply_id, comment_id, user_id,  
        description, image, audio,
        gif, created_at)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, toTimestamp(now()));
"#;

pub static CREATE_INACTIVE_COMMENT_REPLY: &str = r#"
    INSERT INTO comments.inactive_comments_reply (post_id, reply_id, comment_id, user_id,  
        description, image, audio, 
        gif, created_at)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, toTimestamp(now()));
"#;

pub static GET_COMMENT_REPLY_BY_REPLY_ID: &str = r#"
    SELECT 
        post_id, 
        reply_id,
        comment_id,
        user_id,
        description,
        image,
        audio,
        gif,
        created_at 
    FROM comments.comments_reply WHERE post_id = ? AND comment_id = ? AND reply_id = ?;
"#;

pub static GET_COMMENTS_REPLY_BY_COMMENT_ID: &str = r#"
    SELECT 
        post_id,
        reply_id,
        comment_id,
        user_id,
        description,
        image,
        audio,
        gif,
        created_at 
    FROM comments.comments_reply WHERE post_id = ? AND comment_id = ?;
"#;

pub static DELETE_COMMENT_REPLY: &str = r#"
    DELETE FROM comments.comments_reply
    WHERE post_id = ? AND comment_id = ? AND reply_id = ?;
"#;

pub static DELETE_COMMENT_REPLIES_BY_COMMENT_ID: &str = r#"
    DELETE FROM comments.comments_reply
    WHERE post_id = ? AND comment_id = ?;
"#;

pub static GET_INACTIVE_REPLY_COMMENT_BY_ID: &str = r#"
    SELECT 
        post_id, 
        reply_id,
        comment_id,
        user_id,
        description,
        image,
        audio,
        gif,
        created_at 
    FROM comments.inactive_comments_reply WHERE post_id = ? AND comment_id = ? AND reply_id = ?;
"#;

pub static DELETE_INACTIVE_REPLY_COMMENT: &str = r#"
    DELETE FROM comments.inactive_comments_reply
    WHERE post_id = ? AND comment_id = ? AND reply_id = ?;
"#;

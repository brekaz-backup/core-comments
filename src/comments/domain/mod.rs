mod comment;
mod comment_redis_repository;
mod comment_redis_repository_interface;
mod comment_repository;
mod comment_repository_interface;

pub use {
    comment::*, comment_redis_repository::CommentRedisRepository,
    comment_redis_repository_interface::CommentRedisRepositoryInterface,
    comment_repository::CommentRepository,
    comment_repository_interface::CommentRepositoryInterface,
};

pub static CREATE_COMMENT: &str = r#"
    INSERT INTO comments.comments (comment_id, post_id, user_id,  
        description, image, audio, 
        gif, created_at)
    VALUES (?, ?, ?, ?, ?, ?, ?, toTimestamp(now()));
"#;

pub static CREATE_INACTIVE_COMMENT: &str = r#"
    INSERT INTO comments.inactive_comments (comment_id, post_id, user_id,  
        description, image, audio, 
        gif, created_at)
    VALUES (?, ?, ?, ?, ?, ?, ?, toTimestamp(now()));
"#;

pub static GET_COMMENTS_BY_POST_ID: &str = r#"
    SELECT comment_id,
        post_id,
        user_id,
        description,
        image,
        audio,
        gif,
        created_at 
    FROM comments.comments WHERE post_id = ?;
"#;

pub static GET_COMMENT_BY_COMMENT_ID: &str = r#"
    SELECT comment_id,
        post_id,
        user_id,
        description,
        image,
        audio,
        gif,
        created_at 
    FROM comments.comments WHERE post_id = ? AND comment_id = ?;
"#;

pub static DELETE_COMMENT: &str = r#"
    DELETE FROM comments.comments 
    WHERE post_id = ? AND comment_id = ?;
"#;

pub static GET_INACTIVE_COMMENT_BY_ID: &str = r#"
    SELECT comment_id,
        post_id,
        user_id,
        description,
        image,
        audio,
        gif,
        created_at 
    FROM comments.inactive_comments WHERE post_id = ? AND comment_id = ?;
"#;

pub static DELETE_INACTIVE_COMMENT: &str = r#"
    DELETE FROM comments.inactive_comments 
    WHERE post_id = ? AND comment_id = ?;
"#;

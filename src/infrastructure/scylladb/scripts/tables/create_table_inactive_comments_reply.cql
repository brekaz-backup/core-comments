CREATE TABLE IF NOT EXISTS comments.inactive_comments_reply (
    reply_id UUID,
    post_id UUID,
    comment_id UUID,
    user_id UUID,
    description TEXT,
    image TEXT,
    audio TEXT,
    gif TEXT,
    created_at TIMESTAMP,
    PRIMARY KEY( (post_id, comment_id), reply_id)
);

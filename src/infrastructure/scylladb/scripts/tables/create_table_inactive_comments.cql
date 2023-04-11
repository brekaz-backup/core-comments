CREATE TABLE IF NOT EXISTS comments.inactive_comments (
  comment_id UUID,
  post_id UUID,
  user_id UUID,
  description TEXT,
  image TEXT,
  audio TEXT,
  gif TEXT,
  created_at TIMESTAMP,
  PRIMARY KEY(post_id, comment_id)
);

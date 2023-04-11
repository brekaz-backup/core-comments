#[derive(Clone)]
pub enum FileTypeEntity {
    Image,
    Audio,
}

#[derive(Clone)]
pub enum CommentTypeEntity {
    Comment,
    Reply,
}

#[derive(Clone)]
pub struct CommentUploadEntity {
    pub parent_id: String,
    pub comment_id: String,
    pub reply_id: Option<String>,
    pub file_key: String,
    pub file_type: FileTypeEntity,
    pub comment_type: CommentTypeEntity,
}

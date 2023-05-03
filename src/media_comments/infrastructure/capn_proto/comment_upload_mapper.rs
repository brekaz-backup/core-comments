use super::models::{CommentTypeEntity, CommentUploadEntity, FileTypeEntity};
use super::schema::comment_upload_capnp::comment_upload::{
    self as CommentUploadProto, CommentType, FileType,
};
use anyhow::Result;

pub struct CommentUploadMapper;

impl CommentUploadMapper {
    pub async fn _proto(entity: &CommentUploadEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<CommentUploadProto::Builder>();

        proto.set_parent_id(&entity.parent_id);
        proto.set_comment_id(&entity.comment_id);
        if let Some(reply_id) = &entity.reply_id {
            proto.set_reply_id(reply_id);
        }
        proto.set_file_key(&entity.file_key);
        proto.set_file_type(match &entity.file_type {
            FileTypeEntity::Image => FileType::Image,
            FileTypeEntity::Audio => FileType::Audio,
        });
        proto.set_comment_type(match &entity.comment_type {
            CommentTypeEntity::Comment => CommentType::Comment,
            CommentTypeEntity::Reply => CommentType::Reply,
        });

        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message)?;
        return Ok(buf_slice);
    }

    pub async fn event(payload: &[u8]) -> Result<CommentUploadEntity> {
        let message_reader =
            capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let message = message_reader.get_root::<CommentUploadProto::Reader>()?;
        Ok(CommentUploadEntity {
            parent_id: message.get_parent_id()?.to_owned(),
            comment_id: message.get_comment_id()?.to_owned(),
            reply_id: if message.has_reply_id() {
                Some(message.get_reply_id()?.to_owned())
            } else {
                None
            },
            file_key: message.get_file_key()?.to_owned(),
            file_type: match message.get_file_type()? {
                FileType::Image => FileTypeEntity::Image,
                FileType::Audio => FileTypeEntity::Audio,
            },
            comment_type: match message.get_comment_type()? {
                CommentType::Comment => CommentTypeEntity::Comment,
                CommentType::Reply => CommentTypeEntity::Reply,
            },
        })
    }
}

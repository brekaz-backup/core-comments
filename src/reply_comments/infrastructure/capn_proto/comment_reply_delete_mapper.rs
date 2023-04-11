use super::models::CommentReplyDeleteEntity;
use super::schema::comment_capnp::comment_reply_delete as CommentReplyDeleteProto;
use anyhow::Result;
use uuid::Uuid;

pub struct CommentReplyDeleteMapper;

impl CommentReplyDeleteMapper {
    pub async fn proto(entity: &CommentReplyDeleteEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<CommentReplyDeleteProto::Builder>();

        proto.set_post_id(&entity.post_id.to_string());
        proto.set_user_id(&entity.user_id.to_string());
        proto.set_comment_id(&entity.comment_id.to_string());
        proto.set_reply_id(&entity.reply_id.to_string());
        proto.set_post_owner_id(&entity.post_owner_id.to_string());

        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message).unwrap();
        return Ok(buf_slice);
    }

    pub async fn _entity(payload: &[u8]) -> Result<CommentReplyDeleteEntity> {
        let message_reader =
            capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let message = message_reader.get_root::<CommentReplyDeleteProto::Reader>()?;
        Ok(CommentReplyDeleteEntity {
            post_id: Uuid::parse_str(message.get_post_id()?)?,
            user_id: Uuid::parse_str(message.get_user_id()?)?,
            comment_id: Uuid::parse_str(message.get_comment_id()?)?,
            reply_id: Uuid::parse_str(message.get_reply_id()?)?,
            post_owner_id: Uuid::parse_str(message.get_post_owner_id()?)?,
        })
    }
}

use super::models::ReplyCountsDeleteEntity;
use super::schema::comment_counts_capnp::reply_counts_delete as CommentReplyCountsDeleteProto;
use anyhow::Result;
use uuid::Uuid;

pub struct ReplyCountsDeleteMapper;

impl ReplyCountsDeleteMapper {
    pub async fn proto(entity: &ReplyCountsDeleteEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<CommentReplyCountsDeleteProto::Builder>();

        proto.set_comment_id(&entity.comment_id.to_string());
        proto.set_reply_id(&entity.reply_id.to_string());

        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message).unwrap();
        return Ok(buf_slice);
    }

    pub async fn _entity(payload: &[u8]) -> Result<ReplyCountsDeleteEntity> {
        let message_reader =
            capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let message = message_reader.get_root::<CommentReplyCountsDeleteProto::Reader>()?;
        Ok(ReplyCountsDeleteEntity {
            comment_id: Uuid::parse_str(message.get_comment_id()?)?,
            reply_id: Uuid::parse_str(message.get_reply_id()?)?,
        })
    }
}

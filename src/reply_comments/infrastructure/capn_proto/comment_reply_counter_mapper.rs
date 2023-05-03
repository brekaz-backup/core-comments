use super::models::CommentRepliesCounterEntity;
use super::schema::comment_counts_capnp::comment_reply_counter as CommentReplyCounterProto;
use anyhow::Result;
use uuid::Uuid;

pub struct CommentRepliesCounterMapper;

impl CommentRepliesCounterMapper {
    pub async fn proto(entity: &CommentRepliesCounterEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<CommentReplyCounterProto::Builder>();

        proto.set_comment_id(&entity.comment_id.to_string());
        proto.set_replies_count(entity.replies_count);

        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message)?;
        return Ok(buf_slice);
    }

    pub async fn _entity(payload: &[u8]) -> Result<CommentRepliesCounterEntity> {
        let message_reader =
            capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let message = message_reader.get_root::<CommentReplyCounterProto::Reader>()?;
        Ok(CommentRepliesCounterEntity {
            comment_id: Uuid::parse_str(message.get_comment_id()?)?,
            replies_count: message.get_replies_count(),
        })
    }
}

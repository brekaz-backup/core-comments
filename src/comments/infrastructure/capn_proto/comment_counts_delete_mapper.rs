use super::models::CommentCountsDeleteEntity;
use super::schema::comment_counts_capnp::comment_counts_delete as CommentCountDeleteProto;
use anyhow::Result;
use uuid::Uuid;

pub struct CommentCountsDeleteMapper;

impl CommentCountsDeleteMapper {
    pub async fn proto(entity: &CommentCountsDeleteEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<CommentCountDeleteProto::Builder>();

        proto.set_post_id(&entity.post_id.to_string());
        proto.set_comment_id(&entity.comment_id.to_string());

        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message)?;
        return Ok(buf_slice);
    }

    pub async fn _entity(payload: &[u8]) -> Result<CommentCountsDeleteEntity> {
        let message_reader =
            capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let message = message_reader.get_root::<CommentCountDeleteProto::Reader>()?;
        Ok(CommentCountsDeleteEntity {
            post_id: Uuid::parse_str(message.get_post_id()?)?,
            comment_id: Uuid::parse_str(message.get_comment_id()?)?,
        })
    }
}

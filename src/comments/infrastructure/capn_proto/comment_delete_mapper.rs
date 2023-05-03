use super::models::CommentDeleteEntity;
use super::schema::comment_capnp::comment_delete as CommentDeleteProto;
use anyhow::Result;
use uuid::Uuid;

pub struct CommentDeleteMapper;

impl CommentDeleteMapper {
    pub async fn proto(entity: &CommentDeleteEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<CommentDeleteProto::Builder>();

        proto.set_user_id(&entity.user_id.to_string());
        proto.set_post_id(&entity.post_id.to_string());
        proto.set_post_owner_id(&entity.post_owner_id.to_string());
        proto.set_comment_id(&entity.comment_id.to_string());

        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message)?;
        return Ok(buf_slice);
    }

    pub async fn _entity(payload: &[u8]) -> Result<CommentDeleteEntity> {
        let message_reader =
            capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let message = message_reader.get_root::<CommentDeleteProto::Reader>()?;
        Ok(CommentDeleteEntity {
            user_id: Uuid::parse_str(message.get_user_id()?)?,
            post_id: Uuid::parse_str(message.get_post_id()?)?,
            post_owner_id: Uuid::parse_str(message.get_post_owner_id()?)?,
            comment_id: Uuid::parse_str(message.get_comment_id()?)?,
        })
    }
}

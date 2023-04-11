use super::models::CommentCreateEntity;
use super::schema::comment_capnp::comment as CommentProto;
use anyhow::Result;
use uuid::Uuid;

pub struct CommentMapper;

impl CommentMapper {
    pub async fn proto(entity: &CommentCreateEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<CommentProto::Builder>();

        proto.set_post_id(&entity.post_id.to_string());
        proto.set_post_owner_id(&entity.post_owner_id.to_string());
        proto.set_user_id(&entity.user_id.to_string());
        proto.set_comment_id(&entity.comment_id.to_string());
        if let Some(description) = &entity.description {
            proto.set_description(description);
        }
        if let Some(image) = &entity.image {
            proto.set_image(image);
        }
        if let Some(audio) = &entity.audio {
            proto.set_audio(audio);
        }
        if let Some(gif) = &entity.gif {
            proto.set_gif(gif);
        }
        proto.set_active(entity.active);

        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message).unwrap();
        return Ok(buf_slice);
    }

    pub async fn _entity(payload: &[u8]) -> Result<CommentCreateEntity> {
        let message_reader =
            capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let message = message_reader.get_root::<CommentProto::Reader>()?;
        Ok(CommentCreateEntity {
            post_id: Uuid::parse_str(message.get_post_id()?)?,
            user_id: Uuid::parse_str(message.get_user_id()?)?,
            comment_id: Uuid::parse_str(message.get_comment_id()?)?,
            post_owner_id: Uuid::parse_str(message.get_post_owner_id()?)?,
            description: if message.has_description() {
                Some(message.get_description()?.to_string())
            } else {
                None
            },
            image: if message.has_image() {
                Some(message.get_image()?.to_string())
            } else {
                None
            },
            audio: if message.has_audio() {
                Some(message.get_audio()?.to_string())
            } else {
                None
            },
            gif: if message.has_gif() {
                Some(message.get_gif()?.to_string())
            } else {
                None
            },
            active: message.get_active(),
        })
    }
}

use super::models::CommentCountsEntity;
use super::schema::comment_counts_capnp::comment_counts as CommentCountProto;
use anyhow::Result;
use uuid::Uuid;

pub struct CommentCountsMapper;

impl CommentCountsMapper {
    pub async fn proto(entity: &CommentCountsEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<CommentCountProto::Builder>();

        proto.set_post_id(&entity.post_id.to_string());
        proto.set_comment_id(&entity.comment_id.to_string());
        proto.set_replies_count(entity.replies_count);
        proto.set_reactions_count1(entity.reactions_count_1);
        proto.set_reactions_count2(entity.reactions_count_2);
        proto.set_reactions_count3(entity.reactions_count_3);
        proto.set_reactions_count4(entity.reactions_count_4);
        proto.set_reactions_count5(entity.reactions_count_5);
        proto.set_reactions_count6(entity.reactions_count_6);

        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message).unwrap();
        return Ok(buf_slice);
    }

    pub async fn _entity(payload: &[u8]) -> Result<CommentCountsEntity> {
        let message_reader =
            capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let message = message_reader.get_root::<CommentCountProto::Reader>()?;
        Ok(CommentCountsEntity {
            post_id: Uuid::parse_str(message.get_post_id()?)?,
            comment_id: Uuid::parse_str(message.get_comment_id()?)?,
            replies_count: message.get_replies_count(),
            reactions_count_1: message.get_reactions_count1(),
            reactions_count_2: message.get_reactions_count2(),
            reactions_count_3: message.get_reactions_count3(),
            reactions_count_4: message.get_reactions_count4(),
            reactions_count_5: message.get_reactions_count5(),
            reactions_count_6: message.get_reactions_count6(),
        })
    }
}

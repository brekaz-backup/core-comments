@0x84cadc37ad28914e;

using Rust = import "rust.capnp";
$Rust.parentModule("comments::infrastructure::capn_proto::schema");

struct CommentCounts {
    postId @0 :Text;
    commentId @1 :Text;
    repliesCount @2 :Int16;
    reactionsCount1 @3 :Int16;
    reactionsCount2 @4 :Int16;
    reactionsCount3 @5 :Int16;
    reactionsCount4 @6 :Int16;
    reactionsCount5 @7 :Int16;
    reactionsCount6 @8 :Int16;
}

struct CommentCountsDelete {
    postId @0 :Text;
    commentId @1 :Text;
}

struct CommentReplyCounter {
    commentId @0 :Text;
    repliesCount @1 :Int16;
}


struct ReplyCounts {
    commentId @0 :Text;
    replyId @1 :Text;
    reactionsCount1 @2 :Int16;
    reactionsCount2 @3 :Int16;
    reactionsCount3 @4 :Int16;
    reactionsCount4 @5 :Int16;
    reactionsCount5 @6 :Int16;
    reactionsCount6 @7 :Int16;
}

struct ReplyCountsDelete{
    commentId @0 :Text;
    replyId @1 :Text;
}

struct ReplyReactionCounter {
    commentId @0 :Text;
    replyId @1 :Text;
    reactionId @2 :Int16;
    reactionsCount @3 :Int16;
}
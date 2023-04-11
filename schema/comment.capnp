@0xe6b07715e355d0fb;

using Rust = import "rust.capnp";
$Rust.parentModule("comments::infrastructure::capn_proto::schema");

struct Comment {
    postId @0 :Text;
    postOwnerId @1 :Text;
    userId @2 :Text;
    commentId @3 :Text;
    description @4 :Text;
    image @5 :Text;
    audio @6 :Text;
    gif @7 :Text;
    active @8 :Bool;
}

struct CommentDelete {
    userId @0 :Text;
    postId @1 :Text;
    postOwnerId @2 :Text;
    commentId @3 :Text;
}

struct CommentReply {
    postId @0 :Text;
    postOwnerId @1 :Text;
    userId @2 :Text;
    commentId @3 :Text;
    commentOwnerId @4 :Text;
    replyId @5 :Text;
    description @6 :Text;
    image @7 :Text;
    audio @8 :Text;
    gif @9 :Text;
    active @10 :Bool;
}

struct CommentReplyDelete {
    postId @0 :Text;
    postOwnerId @1 :Text;
    userId @2 :Text;
    commentId @3 :Text;
    replyId @4 :Text;
}
@0xeefdf1ff4054ffd3;

using Rust = import "rust.capnp";
$Rust.parentModule("media_comments::infrastructure::capn_proto::schema");

struct CommentUpload {
    parentId @0 :Text;
    commentId @1 :Text;
    replyId @2 :Text;
    fileKey @3 :Text;
    fileType @4 :FileType;
    commentType @5 :CommentType;

    enum FileType {
        image @0;
        audio @1;
    }

    enum CommentType {
        comment @0;
        reply @1;
    }
}
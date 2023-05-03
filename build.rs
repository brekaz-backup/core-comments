extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new()
        .file("schema/rust.capnp")
        .file("schema/comment.capnp")
        .file("schema/comment_upload.capnp")
        .file("schema/comment_counts.capnp")
        .run()
        .expect("compiling cap'n proto schema");
}

#[allow(dead_code, unused)]
extern crate capnp;

#[allow(dead_code, unused)]
pub mod comment_upload_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/comment_upload_capnp.rs"));
}

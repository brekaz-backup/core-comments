#[allow(dead_code, unused)]
extern crate capnp;

#[allow(dead_code, unused)]
pub mod comment_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/comment_capnp.rs"));
}

#[allow(dead_code, unused)]
pub mod comment_counts_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/comment_counts_capnp.rs"));
}

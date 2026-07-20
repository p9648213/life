pub mod constant;
pub mod http;
pub mod server;
pub mod util;

pub mod templates {
    include!(concat!(env!("OUT_DIR"), "/templates.rs"));
}

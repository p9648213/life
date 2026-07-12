pub mod html;
pub mod http;
pub mod server;

pub mod templates {
    include!(concat!(env!("OUT_DIR"), "/templates.rs"));
}


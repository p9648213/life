pub mod app;
pub mod constant;
pub mod http;
pub mod route;
pub mod server;
pub mod state;
pub mod storage;
pub mod util;
pub mod templates {
    include!(concat!(env!("OUT_DIR"), "/templates.rs"));
}

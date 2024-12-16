pub mod config;
pub mod constant;
pub mod controller;
pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/tutorial.rs"));
}

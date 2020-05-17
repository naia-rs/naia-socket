
#[macro_use]
extern crate log;

pub mod find_available_port;
pub mod find_my_ip_address;

mod constants;
pub use constants::{SERVER_HANDSHAKE_MESSAGE, CLIENT_HANDSHAKE_MESSAGE};

#[macro_use]
extern crate log;

#[cfg(not(target_arch = "wasm32"))]
mod udp_client_socket;
#[cfg(target_arch = "wasm32")]
mod webrtc_client_socket;

mod socket_event;//error
mod message_sender;
mod client_socket;

pub use client_socket::{ClientSocket, ClientSocketImpl};
pub use socket_event::{SocketEvent};
pub use message_sender::{MessageSender};
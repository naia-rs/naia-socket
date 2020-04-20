
mod client_socket;
use client_socket::ClientSocket;
use std::net::IpAddr;
use async_trait::async_trait;

#[async_trait]
pub trait ServerSocket {
    fn new() -> Self;

    async fn listen(&self, address: &str);

    fn on_connection(&mut self, func: impl Fn(&ClientSocket) + Sync + 'static);

    fn on_receive(&mut self, func: impl Fn(&ClientSocket, &str) + Sync + 'static);

    fn on_error(&mut self, func: impl Fn(&ClientSocket, &str) + Sync + 'static);

    fn on_disconnection(&mut self, func: impl Fn(&ClientSocket) + Sync + 'static);
}

/// Proto Linux Server
#[cfg(feature = "UdpServer")]
mod udp_server_socket;

#[cfg(feature = "UdpServer")]
pub use self::udp_server_socket::UdpServerSocket;

#[cfg(feature = "UdpServer")]
pub type ServerSocketImpl = UdpServerSocket;

/// Final Server ///
#[cfg(feature = "WebrtcServer")]
mod webrtc_server_socket;

#[cfg(feature = "WebrtcServer")]
pub use self::webrtc_server_socket::WebrtcServerSocket;

#[cfg(feature = "WebrtcServer")]
pub type ServerSocketImpl = WebrtcServerSocket;
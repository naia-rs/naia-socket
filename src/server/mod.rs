
use gaia_socket::{ServerSocket, ServerSocketImpl};
use std::net::{SocketAddr, IpAddr};
use crate::internal_shared::find_ip_address;

const DEFAULT_PORT: &str = "3179";

pub struct Server {
    //socket: ServerSocketImpl
}

impl Server {
    pub async fn new() -> Server { //args should take a shared config, and a port

        println!("Server New!");

        let mut server_socket = ServerSocketImpl::new();

        server_socket.on_connection(|client_socket| {
            println!("Server on_connection(), connected to {}", client_socket.ip);

            let msg: String = "hello new client!".to_string();
            client_socket.send(msg.as_str());
        });

        server_socket.on_receive(|client_socket, msg| {
            println!("Server on_receive(): {:?}", msg);
            println!("sending: {}", msg);
            //let response_msg = "echo from server: ".to_owned() + msg;
            client_socket.send(msg);
        });

        server_socket.on_disconnection(|client_socket| {
            println!("Server on_disconnection(): {:?}", client_socket.ip);
        });

        let current_socket_address = find_ip_address::get() + ":" + DEFAULT_PORT;
        server_socket.listen(current_socket_address.as_str())
            .await;

        Server {
            //socket: server_socket
        }
    }

    pub fn update(&mut self) {

    }

    pub fn connect(&self, listen_addr: SocketAddr) { //put a port in here..

    }

    pub fn on_connect(&self, func: fn()) { //function should have client, clientData, and callback?

    }

    pub fn on_disconnect(&self, func: fn()) { //function should have client

    }

    pub fn add_object(&self) {

    }

    pub fn remove_object(&self) {

    }

    pub fn send_message(&self) {

    }

    pub fn receive_message(&self) {
    }
}

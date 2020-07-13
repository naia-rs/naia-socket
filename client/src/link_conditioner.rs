use naia_socket_shared::LinkConditionerConfig;

use super::{
    client_socket::ClientSocketTrait, error::NaiaClientSocketError, message_sender::MessageSender,
    socket_event::SocketEvent,
};

pub struct LinkConditioner {
    config: LinkConditionerConfig,
    inner_socket: Box<dyn ClientSocketTrait>,
}

impl LinkConditioner {
    pub fn new(config: &LinkConditionerConfig, socket: Box<dyn ClientSocketTrait>) -> Self {
        LinkConditioner {
            config: config.clone(),
            inner_socket: socket,
        }
    }
}

impl ClientSocketTrait for LinkConditioner {
    fn receive(&mut self) -> Result<SocketEvent, NaiaClientSocketError> {
        loop {
            match self.inner_socket.receive() {
                Ok(event) => match event {
                    SocketEvent::None => {
                        break;
                    }
                    SocketEvent::Packet(packet) => {
                        self.process_result(Ok(SocketEvent::Packet(packet)));
                    }
                },
                Err(error) => {
                    self.process_result(Err(error));
                }
            }
        }

        if self.has_result() {
            self.get_result()
        } else {
            Ok(SocketEvent::None)
        }
    }

    fn get_sender(&mut self) -> MessageSender {
        self.inner_socket.get_sender()
    }

    fn with_link_conditioner(
        self: Box<Self>,
        config: &LinkConditionerConfig,
    ) -> Box<dyn ClientSocketTrait> {
        // Absolutely do not recommend decorating a socket with multiple link
        // conditioners... why would you do this??
        Box::new(LinkConditioner::new(config, self))
    }
}

impl LinkConditioner {
    fn process_result(&mut self, result: Result<SocketEvent, NaiaClientSocketError>) {
        unimplemented!()
    }

    fn has_result(&self) -> bool {
        unimplemented!()
    }

    fn get_result(&mut self) -> Result<SocketEvent, NaiaClientSocketError> {
        unimplemented!()
    }
}
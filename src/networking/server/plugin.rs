use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{networking::server::{resources::*, systems::*}, BUFFER_SIZE};

#[derive(Default)]
pub struct ServerNetworkingPlugin;

impl Plugin for ServerNetworkingPlugin {
    fn build(&self, app: &mut App) {
        let socket = UdpSocket::bind("0.0.0.0:34243").unwrap();
        socket.set_nonblocking(true).unwrap();
        app.insert_resource(socket)
            .insert_resource([0_u8; BUFFER_SIZE])
            .init_resource::<Clients>()
            .init_resource::<DownstreamBuffer>()
            .add_system_to_stage(CoreStage::First, upstream_receiver_writer)
            .add_system_to_stage(CoreStage::Last, downstream_reader)
            .add_system_to_stage(CoreStage::Last, downstream_sender.after(downstream_reader));
    }
}
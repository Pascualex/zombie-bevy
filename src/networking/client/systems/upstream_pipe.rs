use std::{io::Write, net::TcpStream};

use bevy::prelude::*;

use crate::protocol::{events::UpstreamEvent, messages::UpstreamMessage};

pub fn upstream_pipe(mut reader: EventReader<UpstreamEvent>, mut sender: ResMut<TcpStream>) {
    if let Some(event) = reader.iter().last() {
        let msg = UpstreamMessage::new(event.data.clone());
        let bytes = bincode::serialize(&msg).unwrap();
        sender.write_all(&bytes).ok();
    }
}
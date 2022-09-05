use std::{io::Write, net::TcpStream};

use bevy::prelude::*;

use crate::protocol::{
    events::{GameDownstreamEvent, LobbyDownstreamEvent},
    messages::DownstreamMessage,
};

pub fn downstream_pipe(
    mut lobby_reader: EventReader<LobbyDownstreamEvent>,
    mut game_reader: EventReader<GameDownstreamEvent>,
    mut streams: ResMut<Vec<TcpStream>>,
) {
    let lobby_events = lobby_reader.iter().cloned().collect();
    let game_events = game_reader.iter().cloned().collect();
    let msg = DownstreamMessage::new(lobby_events, game_events);
    let bytes = bincode::serialize(&msg).unwrap();
    for stream in streams.iter_mut() {
        stream.write_all(&bytes).ok();
    }
}
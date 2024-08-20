use pumpkin_macros::packet;

use crate::{position::WorldPosition, VarInt};

#[derive(serde::Deserialize)]
#[allow(dead_code)]
#[packet(0x24)]
pub struct SPlayerAction {
    status: VarInt,
    location: WorldPosition,
    face: u8,
    sequence: VarInt,
}
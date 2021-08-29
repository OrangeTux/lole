pub mod event;
pub mod motion;
pub mod participants;

use crate::error::{ErrorKind, ParseError};
use crate::frame::participants::ParticipantsBody;
use crate::frame::{event::EventBody, motion::MotionBody};
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
pub struct Frame {
    pub header: Header,
    pub body: Data,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Header {
    pub packet_format: u16,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,
    pub packet_id: PacketType,
    pub session_uid: u64,
    pub session_time: f32,
    pub frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: u8,
}

/// The F1 2020 API defines 10 different types of packets.
#[derive(Debug, PartialEq, Clone)]
pub enum PacketType {
    /// This packet contains physics data for all cars being driven.
    Motion,

    /// This packet contains details of the session in progress.
    Session,

    /// The lap data packet gives details of all the cars in the session.
    LapData,

    /// This packet gives details of events that happen during the course of a session.
    Event,

    /// This is a list of participants in the race.
    Participants,

    /// This packet details the car setups for each vehicle in the session.
    CarSetups,

    /// This packet details telemetry for all the cars in the race
    CarTelemetry,

    /// This packet details car statuses for all the cars in the race.
    CarStatus,

    /// This packet details the final classification at the end of the race,
    FinalClassification,

    /// This packet details the players currently in a multiplayer lobby.
    LobbyInfo,
}

impl TryFrom<u8> for PacketType {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Motion),
            1 => Ok(Self::Session),
            2 => Ok(Self::LapData),
            3 => Ok(Self::Event),
            4 => Ok(Self::Participants),
            5 => Ok(Self::CarSetups),
            6 => Ok(Self::CarTelemetry),
            7 => Ok(Self::CarStatus),
            8 => Ok(Self::FinalClassification),
            9 => Ok(Self::LobbyInfo),
            _ => Err(ParseError::new(ErrorKind::InvalidPacketType(value))),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    Event(EventBody),
    Motion(MotionBody),
    Participants(ParticipantsBody),
}

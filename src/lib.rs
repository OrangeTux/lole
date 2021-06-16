use nom::{
    combinator::map_res,
    number::complete::{le_f32, le_u16, le_u32, le_u64, le_u8},
    IResult,
};
use std::convert::TryFrom;

/// The F1 2020 API defines 10 different types of packets.
#[derive(Debug, PartialEq)]
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
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Motion),
            2 => Ok(Self::Session),
            3 => Ok(Self::LapData),
            4 => Ok(Self::Event),
            5 => Ok(Self::Participants),
            6 => Ok(Self::CarSetups),
            7 => Ok(Self::CarTelemetry),
            8 => Ok(Self::CarStatus),
            9 => Ok(Self::FinalClassification),
            10 => Ok(Self::LobbyInfo),
            _ => Err("Invalid value"),
        }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Frame {
    pub header: Header,
}

pub fn header(input: &'static [u8]) -> IResult<&'static [u8], Header> {
    let (input, packet_format) = le_u16(input)?;
    let (input, game_major_version) = le_u8(input)?;
    let (input, game_minor_version) = le_u8(input)?;
    let (input, packet_version) = le_u8(input)?;
    let (input, packet_id) = map_res(le_u8, PacketType::try_from)(input)?;
    let (input, session_uid) = le_u64(input)?;
    let (input, session_time) = le_f32(input)?;
    let (input, frame_identifier) = le_u32(input)?;
    let (input, player_car_index) = le_u8(input)?;
    let (input, secondary_player_car_index) = le_u8(input)?;

    let header = Header {
        packet_format,
        game_major_version,
        game_minor_version,
        packet_version,
        packet_id,
        session_uid,
        session_time,
        frame_identifier,
        player_car_index,
        secondary_player_car_index,
    };
    Ok((input, header))
}

#[test]
fn test_packet_format() {
    let data: &[u8; 35]= b"\xe4\x07\x01\x12\x01\x03\t)\xb4\xdf8P\x1e\x91\x00\x00\x00\x00\x00\x00\x00\x00\x13\xffSSTA\xc8X\xf5/\x00\x00\x00";
    let (_input, header) = header(data).unwrap();
    assert_eq!(
        header,
        Header {
            packet_format: 2020,
            game_major_version: 1,
            game_minor_version: 18,
            packet_version: 1,
            packet_id: PacketType::LapData,
            session_uid: 10456883590002387209,
            session_time: 0.0,
            frame_identifier: 0,
            player_car_index: 19,
            secondary_player_car_index: 255,
        }
    );
}

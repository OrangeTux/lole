use nom::{
    bytes::complete::tag,
    number::complete::{le_f32, le_u16, le_u32, le_u64, le_u8},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Header {
    pub packet_format: u16,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,
    pub packet_id: u8,
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
    let (input, packet_id) = le_u8(input)?;
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
            packet_id: 3,
            session_uid: 10456883590002387209,
            session_time: 0.0,
            frame_identifier: 0,
            player_car_index: 19,
            secondary_player_car_index: 255,
        }
    );
}

use crate::frame::{
    event::{EventBody, EventDetails, InfringementType, PenaltyType},
    motion::{CarMotion, MotionBody},
    Data, Frame, Header, PacketType,
};
use nom::{
    bytes::complete::take,
    combinator::map_res,
    error::VerboseError,
    number::complete::{le_f32, le_u16, le_u32, le_u64, le_u8},
    IResult,
};
use std::convert::TryFrom;

pub fn frame(input: &[u8]) -> IResult<&[u8], Frame, VerboseError<&[u8]>> {
    let (input, header) = header(input)?;
    let (input, body) = match header.packet_id {
        PacketType::Event => {
            let (input, body) = event_body(input)?;
            (input, Data::Event(body))
        }
        PacketType::Motion => {
            let (input, body) = motion_body(input)?;
            (input, Data::Motion(body))
        }
        _ => {
            return Err(nom::Err::Error(nom::error::make_error(
                input,
                nom::error::ErrorKind::NoneOf,
            )))
        }
    };

    Ok((input, Frame { header, body }))
}

/// Parse byte slice as `Header`.
pub fn header(input: &[u8]) -> IResult<&[u8], Header, VerboseError<&[u8]>> {
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

/// Parse byte slice as `EventBody`.
pub fn event_body(input: &[u8]) -> IResult<&[u8], EventBody, VerboseError<&[u8]>> {
    let (input, code) = map_res(take(4usize), std::str::from_utf8)(input)?;
    let (input, details) = match code {
        "SSTA" => (input, EventDetails::SessionStarted),
        "SEND" => (input, EventDetails::SessionEnded),
        "FTLP" => {
            let (input, vehicle_id) = le_u8(input)?;
            let (input, lap_time) = le_f32(input)?;

            (
                input,
                EventDetails::FastestLap {
                    lap_time,
                    vehicle_id,
                },
            )
        }
        "RTMT" => {
            let (input, vehicle_id) = le_u8(input)?;
            (input, EventDetails::Retirement { vehicle_id })
        }
        "DRSE" => (input, EventDetails::DRSEnabled),
        "DRSD" => (input, EventDetails::DRSDisabled),
        "TMPT" => {
            let (input, vehicle_id) = le_u8(input)?;
            (input, EventDetails::TeamMateInPits { vehicle_id })
        }
        "CHQF" => (input, EventDetails::ChequeredFlag),
        "RCWN" => (input, EventDetails::RaceWinner),
        "PENA" => {
            let (input, penalty_type) = map_res(le_u8, PenaltyType::try_from)(input)?;
            let (input, infringement_type) = map_res(le_u8, InfringementType::try_from)(input)?;
            let (input, vehicle_id) = le_u8(input)?;
            let (input, other_vehicle_id) = le_u8(input)?;
            let (input, time) = le_u8(input)?;
            let (input, lap_number) = le_u8(input)?;
            let (input, places_gained) = le_u8(input)?;

            (
                input,
                EventDetails::Penalty {
                    penalty_type,
                    infringement_type,
                    vehicle_id,
                    other_vehicle_id,
                    time,
                    lap_number,
                    places_gained,
                },
            )
        }
        "SPTP" => {
            let (input, vehicle_id) = le_u8(input)?;
            let (input, speed) = le_f32(input)?;
            (input, EventDetails::SpeedTrap { vehicle_id, speed })
        }
        _ => {
            return Err(nom::Err::Error(nom::error::make_error(
                input,
                nom::error::ErrorKind::NoneOf,
            )))
        }
    };
    Ok((
        input,
        EventBody {
            code: code.to_string(),
            details,
        },
    ))
}

pub fn motion_body(input: &[u8]) -> IResult<&[u8], MotionBody, VerboseError<&[u8]>> {
    let mut car_motions: [CarMotion; 22] = [CarMotion::default(); 22];
    for n in 0..22 {
        let (_, motion) = car_motion(input)?;
        car_motions[n] = motion;
    }

    Ok((
        input,
        MotionBody {
            car_motion: car_motions,
        },
    ))
}

pub fn car_motion(input: &[u8]) -> IResult<&[u8], CarMotion, VerboseError<&[u8]>> {
    let (input, world_position_x) = le_f32(input)?;
    let (input, world_position_y) = le_f32(input)?;
    let (input, world_position_z) = le_f32(input)?;
    let (input, world_velocity_x) = le_f32(input)?;
    let (input, world_velocity_y) = le_f32(input)?;
    let (input, world_velocity_z) = le_f32(input)?;
    let (input, world_forward_direction_x) = le_u16(input)?;
    let (input, world_forward_direction_y) = le_u16(input)?;
    let (input, world_forward_direction_z) = le_u16(input)?;
    let (input, world_right_direction_x) = le_u16(input)?;
    let (input, world_right_direction_y) = le_u16(input)?;
    let (input, world_right_direction_z) = le_u16(input)?;
    let (input, g_force_lateral) = le_f32(input)?;
    let (input, g_force_longitudinal) = le_f32(input)?;
    let (input, g_force_vertical) = le_f32(input)?;
    let (input, yaw) = le_f32(input)?;
    let (input, pitch) = le_f32(input)?;
    let (input, roll) = le_f32(input)?;

    Ok((
        input,
        CarMotion {
            world_position_x,
            world_position_y,
            world_position_z,
            world_velocity_x,
            world_velocity_y,
            world_velocity_z,
            world_forward_direction_x,
            world_forward_direction_y,
            world_forward_direction_z,
            world_right_direction_x,
            world_right_direction_y,
            world_right_direction_z,
            g_force_lateral,
            g_force_longitudinal,
            g_force_vertical,
            yaw,
            pitch,
            roll,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::frame::{
        event::{EventBody, EventDetails, InfringementType, PenaltyType},
        Header, PacketType,
    };

    use crate::parser::{event_body, header};

    #[test]
    fn test_parse_header() {
        let data: &[u8]= b"\xe4\x07\x01\x12\x01\x03\t)\xb4\xdf8P\x1e\x91\x00\x00\x00\x00\x00\x00\x00\x00\x13\xffSSTA\xc8X\xf5/\x00\x00\x00";
        let (_input, _header) = header(data).unwrap();
        assert_eq!(
            _header,
            Header {
                packet_format: 2020,
                game_major_version: 1,
                game_minor_version: 18,
                packet_version: 1,
                packet_id: PacketType::Event,
                session_uid: 10456883590002387209,
                session_time: 0.0,
                frame_identifier: 0,
                player_car_index: 19,
                secondary_player_car_index: 255,
            }
        );
    }

    #[test]
    fn test_parse_event() {
        let data: &[u8] = &[
            228, 7, 1, 19, 1, 3, 207, 82, 48, 29, 211, 221, 97, 126, 56, 88, 29, 66, 196, 1, 0, 0,
            19, 255, 80, 69, 78, 65, 16, 41, 19, 255, 255, 1, 255,
        ];
        let (_input, _header) = header(data).unwrap();
        let (_input, body) = event_body(_input).unwrap();
        assert_eq!(
            body,
            EventBody {
                code: "PENA".to_string(),
                details: EventDetails::Penalty {
                    penalty_type: PenaltyType::Retired,
                    infringement_type: InfringementType::RetiredTerminallyDamaged,
                    vehicle_id: 19,
                    other_vehicle_id: 255,
                    time: 255,
                    lap_number: 1,
                    places_gained: 255,
                }
            }
        )
    }
}

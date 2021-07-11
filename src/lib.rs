use nom::{
    bytes::complete::take,
    combinator::map_res,
    error::VerboseError,
    number::complete::{le_f32, le_u16, le_u32, le_u64, le_u8},
    IResult,
};

use std::convert::TryFrom;

mod error;
use error::{Error, ErrorKind};

/// The F1 2020 API defines 10 different types of packets.
#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PenaltyType {
    /// Penalty that forces driver to drive through the pit lane without stopping.
    DriveThrough,
    /// Penalty that forces driver pit and stop for 10 seconds before engineers
    /// work on the car.
    StopGo,
    /// Penalty that pushes back on the start grid.
    GridPenalty,
    PenaltyReminder,
    TimePenalty,
    Warning,
    Disqualified,
    RemovedFromFormationLap,
    ParkedTooLongTimer,
    /// Penalty given when driver doesn't comply to tire regulations.
    TyreRegulations,
    ThisLapInvalidated,
    ThisAndNextLapInvalided,
    ThisLapInvalidatedWithoutReason,
    ThisAndPreviousLapInvalidated,
    ThisAndPreviousLapInvalidatedWithoutReason,
    Retired,
    BlackFlagTimer,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InfringementType {
    BlockingBySlowDriving,
    BlockingByWrongWayDriving,
    ReversingOffTheStartLine,
    BigCollision,
    SmallCollision,
    CollisionFailedToHandBackPositionSingle,
    CollisionFailedToHandBackPositionMultiple,
    CornerCuttingGainedTime,
    CornerCuttingOvertakeSingle,
    CornerCuttingOvertakeMultiple,
    CrossedPitExitLane,
    IgnoringBlueFlags,
    IgnoringYellowFlags,
    IgnoringDriveThrough,
    TooManyDriveThroughs,
    DriveThroughReminderServerWithinNLaps,
    DriveThroughReminderServeThisLap,
    PitLaneSpeeding,
    ParkedForTooLong,
    IgnoringTyreRegulations,
    TooManyPenalties,
    MultipleWarnings,
    ApproachingDisqualification,
    TyreRegulationsSelectSingle,
    TyreRegulationsSelectMultiple,
    LapInvalidatedCornerCutting,
    LapInvalidatedRunningWide,
    CornerCuttingRanWideGainedTimeMinor,
    CornerCuttingRanWideGainedTimeSignificant,
    CornerCuttingRanWideGainedTimeExtreme,
    LapInvalidatedWallRiding,
    LapInvalidatedFlashbackUsed,
    LapInvalidatedResetToTrack,
    BlockingPitLane,
    JumpStart,
    SafetyCarToCarCollision,
    SafetyCarIllegalOvertake,
    SafetyCarExceedingAllowedPace,
    VirtualSafetyCarExceedingAllowedPace,
    FormationLapBelowAllowedSpeed,
    RetiredMechanicalFailure,
    RetiredTerminallyDamaged,
    SafetyCarFallingTooFarBack,
    BlackFlagTimer,
    UnservedStopGoPenalty,
    UnservedDriveThroughPenalty,
    EngineComponentChange,
    GearboxChange,
    LeagueGridPenalty,
    RetryPenalty,
    IllegalTimeGain,
    MandatoryPitstop,
}

impl TryFrom<u8> for InfringementType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::BlockingBySlowDriving),
            1 => Ok(Self::BlockingByWrongWayDriving),
            2 => Ok(Self::ReversingOffTheStartLine),
            3 => Ok(Self::BigCollision),
            4 => Ok(Self::SmallCollision),
            5 => Ok(Self::CollisionFailedToHandBackPositionSingle),
            6 => Ok(Self::CollisionFailedToHandBackPositionMultiple),
            7 => Ok(Self::CornerCuttingGainedTime),
            8 => Ok(Self::CornerCuttingOvertakeSingle),
            9 => Ok(Self::CornerCuttingOvertakeMultiple),
            10 => Ok(Self::CrossedPitExitLane),
            11 => Ok(Self::IgnoringBlueFlags),
            12 => Ok(Self::IgnoringYellowFlags),
            13 => Ok(Self::IgnoringDriveThrough),
            14 => Ok(Self::TooManyDriveThroughs),
            15 => Ok(Self::DriveThroughReminderServerWithinNLaps),
            16 => Ok(Self::DriveThroughReminderServeThisLap),
            17 => Ok(Self::PitLaneSpeeding),
            18 => Ok(Self::ParkedForTooLong),
            19 => Ok(Self::IgnoringTyreRegulations),
            20 => Ok(Self::TooManyPenalties),
            21 => Ok(Self::MultipleWarnings),
            22 => Ok(Self::MultipleWarnings),
            23 => Ok(Self::ApproachingDisqualification),
            24 => Ok(Self::TyreRegulationsSelectSingle),
            25 => Ok(Self::TyreRegulationsSelectMultiple),
            26 => Ok(Self::LapInvalidatedRunningWide),
            27 => Ok(Self::LapInvalidatedRunningWide),
            28 => Ok(Self::CornerCuttingRanWideGainedTimeMinor),
            29 => Ok(Self::CornerCuttingRanWideGainedTimeSignificant),
            30 => Ok(Self::CornerCuttingRanWideGainedTimeExtreme),
            31 => Ok(Self::LapInvalidatedWallRiding),
            32 => Ok(Self::LapInvalidatedResetToTrack),
            33 => Ok(Self::BlockingPitLane),
            34 => Ok(Self::JumpStart),
            35 => Ok(Self::SafetyCarToCarCollision),
            36 => Ok(Self::SafetyCarIllegalOvertake),
            37 => Ok(Self::SafetyCarExceedingAllowedPace),
            38 => Ok(Self::VirtualSafetyCarExceedingAllowedPace),
            39 => Ok(Self::FormationLapBelowAllowedSpeed),
            40 => Ok(Self::RetiredMechanicalFailure),
            41 => Ok(Self::RetiredTerminallyDamaged),
            42 => Ok(Self::SafetyCarFallingTooFarBack),
            43 => Ok(Self::BlackFlagTimer),
            44 => Ok(Self::UnservedStopGoPenalty),
            45 => Ok(Self::UnservedDriveThroughPenalty),
            46 => Ok(Self::EngineComponentChange),
            47 => Ok(Self::GearboxChange),
            48 => Ok(Self::LeagueGridPenalty),
            49 => Ok(Self::RetryPenalty),
            50 => Ok(Self::IllegalTimeGain),
            51 => Ok(Self::MandatoryPitstop),
            _ => Err(Error::new(ErrorKind::InvalidInfringementType(value))),
        }
    }
}

impl TryFrom<u8> for PenaltyType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DriveThrough),
            1 => Ok(Self::StopGo),
            2 => Ok(Self::GridPenalty),
            3 => Ok(Self::PenaltyReminder),
            4 => Ok(Self::TimePenalty),
            5 => Ok(Self::Warning),
            6 => Ok(Self::Disqualified),
            7 => Ok(Self::RemovedFromFormationLap),
            8 => Ok(Self::ParkedTooLongTimer),
            9 => Ok(Self::TyreRegulations),
            10 => Ok(Self::ThisLapInvalidated),
            11 => Ok(Self::ThisAndNextLapInvalided),
            12 => Ok(Self::ThisLapInvalidatedWithoutReason),
            13 => Ok(Self::ThisAndPreviousLapInvalidated),
            14 => Ok(Self::ThisAndPreviousLapInvalidated),
            15 => Ok(Self::ThisAndPreviousLapInvalidatedWithoutReason),
            16 => Ok(Self::Retired),
            17 => Ok(Self::BlackFlagTimer),
            _ => Err(Error::new(ErrorKind::InvalidPenaltyType(value))),
        }
    }
}

impl TryFrom<u8> for PacketType {
    type Error = Error;

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
            _ => Err(Error::new(ErrorKind::InvalidPacketType(value))),
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

/// Body of a frame.
#[derive(Debug, PartialEq)]
pub struct EventBody {
    /// Code indicating the event type.
    pub code: String,

    /// Details of the event.
    pub details: EventDetails,
}

/// `Enum` representing the details of the `EventBody` frame.
#[derive(Debug, PartialEq)]
pub enum EventDetails {
    /// Event generated when the chequered flag is waived.
    ChequeredFlag,
    /// Event generated when DRS is enabled.
    DRSEnabled,
    /// Event generated when DRS has been disabled.
    DRSDisabled,
    /// Event generated when driver has the fastest lap.
    FastestLap {
        vehicle_id: u8,
        /// Lap time in seconds.
        lap_time: f32,
    },
    /// Event generated when a driver receives a penalty.
    Penalty {
        /// Type of penalty given for the foul.
        penalty_type: PenaltyType,
        /// Type of foul driver committed.
        infringement_type: InfringementType,
        vehicle_id: u8,
        other_vehicle_id: u8,
        time: u8,
        /// Lap number when foul was committed.
        lap_number: u8,
        /// Number of positions gained by the foul.
        places_gained: u8,
    },
    /// Event generated when someone wins the race.
    RaceWinner,
    /// Event generated when a car retires.
    Retirement { vehicle_id: u8 },
    /// Event generated when the session has ended.
    SessionEnded,
    /// Event generated when the session is started.
    SessionStarted,
    /// Event generated when player is hitting the speed track.
    SpeedTrap {
        vehicle_id: u8,
        /// Top speed in km/h.
        speed: f32,
    },
    /// Event generated when team mate enters the pit lane.
    TeamMateInPits { vehicle_id: u8 },
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
        228, 7, 1, 19, 1, 3, 207, 82, 48, 29, 211, 221, 97, 126, 56, 88, 29, 66, 196, 1, 0, 0, 19,
        255, 80, 69, 78, 65, 16, 41, 19, 255, 255, 1, 255,
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

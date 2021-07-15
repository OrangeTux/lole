use crate::error::{ErrorKind, ParseError};
use std::convert::TryFrom;

/// Body of a frame.
#[derive(Debug, PartialEq, Clone)]
pub struct EventBody {
    /// Code indicating the event type.
    pub code: String,

    /// Details of the event.
    pub details: EventDetails,
}

/// `Enum` representing the details of the `EventBody` frame.
#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

impl TryFrom<u8> for PenaltyType {
    type Error = ParseError;

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
            _ => Err(ParseError::new(ErrorKind::InvalidPenaltyType(value))),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    type Error = ParseError;

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
            _ => Err(ParseError::new(ErrorKind::InvalidInfringementType(value))),
        }
    }
}

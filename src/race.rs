use crate::frame::{
    event::{EventBody, EventDetails},
    motion::{CarMotion, MotionBody},
    participants::{Driver, Participant, ParticipantsBody},
    Data, Frame,
};


/// `Race` keeps track of a session.
pub struct Race {
    pub status: Status,
    pub participants: Vec<Participant>,
    pub race_lines: RaceLines,
}

impl Race {
    pub fn new() -> Self {
        Race {
            status: Status::Unknown,
            participants: vec![],
            race_lines: RaceLines { data: vec![] },
        }
    }

    pub fn feed_frame(&mut self, frame: Frame) {
        let timestamp: Timestamp = frame.header.session_time;
        match frame {
            Frame {
                header: _,
                body: Data::Participants(ParticipantsBody { participants, .. }),
            } => {
                self.participants = participants;
            }
            Frame {
                header: _,
                body: Data::Motion(MotionBody { car_motion, .. }),
            } => {
                self.handle_motion(timestamp, car_motion);
            }
            Frame {
                header: _,
                body: Data::Event(EventBody { details, .. }),
            } => self.handle_event(details),
        }
    }

    fn handle_event(&mut self, details: EventDetails) {
        match details {
            EventDetails::SessionStarted => self.status = Status::Unfolding,
            EventDetails::SessionEnded => self.status = Status::Finished,
            _ => {}
        }
    }

    fn handle_motion(&mut self, timestamp: Timestamp, details: [CarMotion; 22]) {
        for (i, motion) in details.iter().enumerate() {
            let driver = self.participants[i].driver_id;
            let point = SpatialLocation {
                driver,
                timestamp,
                coords: (
                    motion.world_position_x,
                    motion.world_position_z,
                    motion.world_position_y,
                ),
            };

            self.race_lines.append(point);
        }
    }
}

/// `RaceLines` abstracts the racing lines drivers race. In other words: it contains
/// the path that the drivers drove across the race track.
pub struct RaceLines {
    data: Vec<SpatialLocation>,
}

impl RaceLines {
    /// Filter `RaceLines` by `Driver`.
    pub fn by_driver(&self, driver: Driver) -> RaceLines {
        let filtered_data = self
            .data
            .iter()
            .cloned()
            .filter(|record| record.driver == driver)
            .collect();
        RaceLines {
            data: filtered_data,
        }
    }

    /// Return `RaceLines` as a `Vec`.
    pub fn to_vec(&self) -> Vec<SpatialLocation> {
        self.data.clone()
    }

    fn append(&mut self, point: SpatialLocation) {
        self.data.push(point);
    }

}

/// `SpatialLocation` contains the physical location of `Driver` at a certain point in time.
#[derive(Debug, Clone, Copy)]
pub struct SpatialLocation {
    pub driver: Driver,
    pub timestamp: Timestamp,
    pub coords: (f32, f32, f32),
}

pub type Timestamp = f32;

/// Status of a `Race`.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Status {
    // Race has ended.
    Finished,
    // Race is on going.
    Unfolding,
    // Status of the race is unknown.
    Unknown,
}

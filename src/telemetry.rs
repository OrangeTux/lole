use crate::error::AppError;
///
/// ```rust
/// use std::net::UdpSocket;
/// use std::thread;
///
/// let socket = UdpSocket::bind("0.0.0.0:20777").unwrap();
/// let mut app = lole::telemetry::App::new(socket);
/// let frames = app.frames();
///
/// thread::spawn(move || {
///     for frame in frames {
///         dbg!(frame);
///     }
/// });
///
/// app.start().unwrap();
/// ```
use crate::frame::Frame;
use crate::parser::frame;

pub struct App {
    socket: std::net::UdpSocket,
    sender: crossbeam_channel::Sender<Frame>,
    receiver: crossbeam_channel::Receiver<Frame>,
}

impl App {
    pub fn new(socket: std::net::UdpSocket) -> App {
        let (s, r) = crossbeam_channel::unbounded();
        App {
            socket,
            sender: s,
            receiver: r,
        }
    }

    // Read bytes from the socket and parse them as a `Frame`. This method loops till it parsed a
    // frame successfully.
    fn read_frame(&mut self) -> Result<Frame, AppError> {
        loop {
            // The biggest frame possible has 1464 bytes.
            let mut buf = [0; 1464];
            let (_, _) = self.socket.recv_from(&mut buf)?;

            // Any error while parsing the frame is silently ignored.
            // That's ugly. But for now I don't know how to handle the
            // `Err(VerboseError<u8>)` decently.
            if let Ok((_, frame)) = frame(&buf) {
                return Ok(frame);
            }
        }
    }

    /// Returns an iterator over the `Frame`s. The iterator only yields frames if
    /// `start` has been called.
    pub fn frames(&self) -> Frames {
        Frames {
            inbound: self.receiver.clone(),
        }
    }

    /// Start reading from the socket.
    pub fn start(&mut self) -> Result<(), crate::error::AppError> {
        loop {
            let frame = self.read_frame()?;

            // Failing to send a message on the channel is fatal. It doesn't
            // make any sense to continue.
            self.sender
                .send(frame)
                .expect("Failed to send frame over channel.");
        }
    }
}

pub struct Frames {
    inbound: crossbeam_channel::Receiver<Frame>,
}

impl Iterator for Frames {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(header) = self.inbound.recv() {
            return Some(header);
        }
        None
    }
}

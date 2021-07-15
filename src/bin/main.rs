use std::net::UdpSocket;
use std::thread;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:20777").expect("Failed to bind to '0.0.0.0:20777'");
    let mut app = lole::telemetry::App::new(socket);

    let frames = app.frames();

    thread::spawn(move || {
        for frame in frames {
            dbg!(frame);
        }
    });

    app.start().expect("Lole crashed.");
}

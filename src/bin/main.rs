use std::net::UdpSocket;
use std::thread;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:20777")?;
    let mut app = Lole::telemetry::App::new(socket);

    let frames = app.frames();

    thread::spawn(move || {
        for frame in frames {
            dbg!(frame);
        }
    });

    app.start();

    Ok(())
}

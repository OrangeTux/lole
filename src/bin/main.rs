use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:20777")?;
    let app = Lole::telemetry::App::new(socket);

    for frame in app {
        dbg!(frame);
    };

    Ok(())
}

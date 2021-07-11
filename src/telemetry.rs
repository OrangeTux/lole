pub struct App {
    socket: std::net::UdpSocket
}

impl App {
    pub fn new(socket: std::net::UdpSocket) -> App {
        App{socket}
    }
}

impl Iterator for App {
    type Item = crate::Header;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 1464];
        let (amt, src) = self.socket.recv_from(&mut buf).expect("Failed to read from socket");
        let (_, header)  = crate::header(&buf).expect("failed to parse header");
        Some(header)
    }
}

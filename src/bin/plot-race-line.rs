use std::io::Write;
use std::thread;
use std::{fs::File, net::UdpSocket};

use lole::race::{Race, Status};
use tera::{Context, Tera};

fn main() {
    let tera = match Tera::new("templates/*.svg") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let socket = UdpSocket::bind("0.0.0.0:20777").expect("Failed to bind to '0.0.0.0:20777'");
    let mut app = lole::telemetry::App::new(socket);
    println!("Started listening on 0.0.0.0:20777.");

    let frames = app.frames();
    thread::spawn(move || app.start().expect("Lole crashed."));

    let mut race = Race::new();
    for frame in frames {
        race.feed_frame(frame);
        if race.status == Status::Finished {
            break;
        }
    }

    let human_driver = race
        .participants
        .iter()
        .find(|participant| participant.ai_controlled == 0)
        .expect("Failed to find driver controlled by a human; all participants AI controlled.")
        .driver_id;

    let data = race
        .race_lines
        .by_driver(human_driver)
        .to_vec()
        .iter()
        .map(|record| (record.coords.0, record.coords.1))
        .collect();

    let svg = plot(tera, data).expect("Failed to plot race line");
    let mut f = File::create("/tmp/track.svg").expect("Failed to open /tmp/track.svg");
    f.write_all(svg.as_bytes())
        .expect("Failed to write to /tmp/track.svg");
    println!("Wrote track to file:///tmp/track.svg");
}

fn plot(template: Tera, coords: Vec<(f32, f32)>) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.insert("coords", &coords[..]);

    template.render("track.svg", &context)
}

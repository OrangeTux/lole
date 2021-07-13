# Lole

Lole is a parser for [telemetry UDP API] that the [F1 2020 game] exposes.
It's named after [Carlos Reutemann], an Argentine racing driver. He's nicknamed "Lole".

## Quick start

First enable the telemetry. Clone the repository, run

``` bash
$ cargo run
```

and start racing!

## Enabling telemetry 

Enable the telemetry in the F1 2020 game. Open the menu `Game Options > Settings > Telemetry Settings`.
Now configure:
* `UDP Telemetry` as `On`
* `UDP Broadcast Mode` as `Off`
* `UDP IP Address` with the IP address of the peer that is running Lole
* `Port` as `20777`
* `UDP Format` as `2020`

## Record and replay telemetry

The [f1-2020-telemetry] package is an alternative implementation written in Python.
This package also provides a few tools to record telemetry and replay it a later point.

Lole includes a recording that can be replayed:

```shell
$ f1-2020-telemetry-player data/recording.sqlite3
2021-07-13 21:26:12.731 | console    | INFO  | Console wait thread started.
2021-07-13 21:26:12.732 | playback   | INFO  | Playback thread started.
...
```

To record your own session, run:

```bash
$ f1-2020-telemetry-recorder
2021-07-13 21:27:06.042 | recorder   | INFO  | Recorder thread started.
2021-07-13 21:27:06.042 | console    | INFO  | Console wait thread started.
2021-07-13 21:27:06.042 | receiver   | INFO  | Receiver thread started, reading UDP packets from port 20777
2021-07-13 21:27:07.002 | recorder   | INFO  | Opening file F1_2019_467dafb619299067.sqlite3
2021-07-13 21:27:07.010 | recorder   | INFO  |     (Created new file.)
2021-07-13 21:27:07.018 | recorder   | INFO  | Recorded 44 packets in 15.872 ms.
2021-07-13 21:27:08.009 | recorder   | INFO  | Recorded 45 packets in 8.290 ms.
...
```

## License

This project is licensed under the [MIT license].

[Carlos Reutemann]: https://en.wikipedia.org/wiki/Carlos_Reutemann
[F1 2020 game]: https://www.codemasters.com/game/f1-2020/
[f1-2020-telemetry]: https://f1-2020-telemetry.readthedocs.io/en/latest/package-documentation.html
[MIT license]: LICENSE
[telemtry UDP API]: https://forums.codemasters.com/topic/50942-f1-2020-udp-specification/

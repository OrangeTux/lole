# Lole

Lole is a parser for [telemetry UDP API] that the [F1 2020 game] exposes.
It's named after [Carlos Reutemann], an Argentine racing driver. He's nicknamed "Lole".

## Quick start

First enable the telemetry. Clone the repository and run

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

## License

This project is licensed under the [MIT license].

[Carlos Reutemann]: https://en.wikipedia.org/wiki/Carlos_Reutemann
[F1 2020 game]: https://www.codemasters.com/game/f1-2020/
[MIT license]: LICENSE.md
[telemtry UDP API]: https://forums.codemasters.com/topic/50942-f1-2020-udp-specification/

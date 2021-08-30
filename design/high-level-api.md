The F1 2021 API exposes a different kind of data in 10 different packet types.
I was thinking to create a low level API that closely follows the API and to create a high
level API.  This high level API should be the primary API for users of this crate. This crate
uses the low level API to build the high level API.

This document explains the struggles I have with designing the high level API. I hope
that writing my struggles down helps me to clear my mind; to design a nice and well thought out
high level API (if that is even possible).

Let's first list what kind of info the F1 API offers:

* timing data related to laps and sectors
* positional data related to track location, acceleration and gravitational forces
* car telemetry about engine rpms, current gear, tyre temperature, fuel mix, tyre wear,
* car setup like aero config, differential setup, brake bias
* driver data like grid position, penalties, FIA flags, number of pit stops,
* events like speed trap, retirement, penalty, start of session
* weather forecast
* track data like length, time left, actual weather

Why do I want to create a high level API?
A lot of packets contain data of all drivers as a list. So you need to know which driver takes
up with position in that list. In other words: you need state to understand those packets. I
don't want users of this crate to keep track of this state themself. It should be offered by
the crate.

The F1 2020 API is big. I'm unsure what info users of this crate are interested in. For now
I'm working on a script to plot the race line of a driver in an SVG. I'll try to come up with
an API that satisfies that use case to keep the problem small.

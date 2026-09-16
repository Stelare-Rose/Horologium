# Horologium
Horologium is a part of the Constellation Project, a set of personal applications designed around my personal workflow. The focus is on time tracking, easily being able to keep track of time backed by a fast local-first data layer.

Horologium is heavily inspired by event-sourcing principles, preferring individual independent events and log reconstruction rather than exact intervals.

# Purpose
Time trackers usually store discrete intervals. Horologium stores independent events (only the start time of each event, and distinct stop events marking "stop tracking"), and reconstructs the event log. This, combined with file-level sync tools (such as [Syncthing](https://syncthing.net/)), allow for a fully local-first time tracker.

# Architecture
Horologium is split into three parts:
 - Command Line Interface  
 The CLI is written in Rust, and is the form that the prototype currently takes. The CLI enables basic recording, tagging, and project assignment, with basic queries on the resulting event log.
 - Background Service
 The service is written in Rust, and handles generation of the event log and verification of the cache.
 - Horologium-Lib
 A Shared Library written in Rust used to standardize commonly used methods across the CLI and Service. This is meant for later development into Android and Desktop interfaces.

The data is intentionally a unidirectional cycle:
```
Data -> Service -> Cache -> Consumer -> Data
```
Note that the Consumer is the UI layer where the user is also expected to write, hence why it writes back to the data.

The underlying file format is [Eridanus](https://git.starrytea.cc/Constellation-Project/Eridanus)

# Platform Support
Currently this project only supports Linux, and specifically a Nix Flake is provided. Android support is planned.

# Roadmap
- [ ] Android Support
- [ ] Desktop with Tauri and Svelte

# License
This project is under the MIT License - see LICENSE file

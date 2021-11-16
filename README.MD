Note management application. The database is a git repository.
===================

Installation and usage of demo application
--------------

Make sure you're using nigthly version of rust:
- `rustup default nightly`

Install cargo-make that takes care of all other dependencies:
- `cargo install cargo-make`

Build and run project using:
- `cargo make --env GIT_REPO=/path/to/git/repo/with/notes app-start`

Eventually terminal will let you know that app is available under http://localhost:4000/


Potentially helpful things
===================

Installing rustup
- `https://rustup.rs/`

Installing the necessary packages on linux
- `sudo apt install build-essential`
- `sudo apt install libssl-dev`
- `sudo apt install pkg-config`
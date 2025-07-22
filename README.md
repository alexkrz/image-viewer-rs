# image-viewer-rs

For getting started I recommend reading the Book [GUI development with Rust and GTK 4](https://gtk-rs.org/gtk4-rs/stable/latest/book/).

The code in this repository is inspired by the `text_viewer` example from <https://github.com/gtk-rs/gtk4-rs/tree/main/examples/text_viewer>.

Our goal is to build a simple cross-platform image viewer that can load an image from the filesystem and display it on the screen.

## Build instructions

First clone the repository to your target platform.

### MacOS

The following has been tested on MacOS:

To build the project simply execute

```bash
cargo run
```

To build a MacOS `.app` bundle, use `cargo-bundle` by running the following:

```bash
cargo add cargo-bundle
cargo bundle --release
```

This will create an `.app` bundle under `target/release/bundle/osx`.

### Windows

Installing GTK4 on Windows is a bad experience..

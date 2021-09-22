# llust-dm
A Display Manager Written in Rust

llust-dm is a display manager using gtk and pam modules. It works inconsistently on Ubuntu and thats all I've tested.

### Usage
- run `cargo build`
- grab the `llust-dm` binary from `target/debug`
- run `llust-dm` with a systemd service file

### Resources
I made llust-dm using [this](https://www.gulshansingh.com/posts/how-to-write-a-display-manager/) tutorial by Gulshan Singh in C, and ported it to Rust.

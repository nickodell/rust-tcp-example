# Rust TCP server example

This is a toy Rust TCP server which implements the [echo protocol](https://en.wikipedia.org/wiki/Echo_Protocol).

Warning - this does minimal error handling.

To run, use `cargo run`.

To test this, you can use the command `while sleep 0.1; do echo foo | timeout 0.1 nc localhost 3333; done` as a client.

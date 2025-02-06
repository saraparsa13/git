# Build Your Own Git in Rust

This project is a minimal Git-like implementation written in Rust.

## Features

- Initialize a new Git-like repository (`init` command)
- Store files as Git blob objects (`hash-object -w` command)
- Retrieve and print stored objects (`cat-file -p` command)

## Installation

To build and run the project, ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

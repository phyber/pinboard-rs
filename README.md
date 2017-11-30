# Pinboard CLI

This project is a command line interface for the [Pinboard](https://pinboard.in)
API.

It is mostly a learning tool for me while I learn Rust, but it may actually
become useful at some point.

## Installation

You may install this project directly from GitHub using the following command:

`cargo install --git <repo url>`

## Configuration

The configuration is currently very simple. Configuration is read from a YAML
file located at `~/.config/pinboard/config.yaml`. The file currently only
contains the API token.

Example:

```yaml
---
api:
    token: 'user:0123456789ABCDEF'
```

## Resources

Since I was using this project to learn more Rust, I figured it may be useful
to list resources that I found useful.

  - Burnt Sushi's [Rust Error Handling](
    http://blog.burntsushi.net/rust-error-handling/) blog post was excellent
    while getting to grips with error handling. This post has since been
    adapted for [The Rust Programming Language](
    https://doc.rust-lang.org/stable/book/first-edition/error-handling.html)
    book.
  - [Rust By Example](https://rustbyexample.com/) for various bits and pieces.
  - [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)
    book.
  - [Rust Clippy](https://github.com/rust-lang-nursery/rust-clippy).
  - Reading the code and documentation of various crates used in the project.
  - Many other things that I've since forgotten about.

## License

This project is released under the MIT license.

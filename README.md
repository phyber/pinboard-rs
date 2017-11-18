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

## License

This project is released under the MIT license.

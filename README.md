# Veloxide-CLI

[![Crates.io](https://img.shields.io/crates/v/veloxide.svg)](https://crates.io/crates/veloxide)
[![Documentation](https://docs.rs/cargo-index-transit/badge.svg)](https://docs.rs/veloxide/)
[![Codecov](https://codecov.io/github/liamwh/veloxide-cli/coverage.svg?branch=main)](https://codecov.io/gh/liamwh/veloxide-cli)
[![Dependency status](https://deps.rs/repo/github/liamwh/veloxide-cli/status.svg)](https://deps.rs/repo/github/liamwh/veloxide-cli)

Command-line utility for Veloxide, the stack for building web apps with Rust.

![Demo GIF](./docs/demo.gif)

More info can be found in the [template repo](https://github.com/liamwh/Veloxide)

## Getting started

Install the pre-requisites:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- [just](https://github.com/casey/just)
- [Protobuf Compiler](https://grpc.io/docs/protoc-installation/)

Install the Veloxide CLI:

```sh
> cargo install veloxide
```

Create your own app:


```zsh
> veloxide init my-app

# Go to the created folder
> cd my-app

# Install the required tools for development
> just install-required

# Start the supporting containers, and then run the app
> just dev

# Once done, open `my-app/` in your IDE

# Happy Coding!
```

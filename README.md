# Veloxide-CLI

Command-line utility for Veloxide, the stack for building web apps with Rust.

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

# Set the environment to use the Postgres config, start the supporting containers, and then run the app
> just dev-postgres

# Once done, open `my-app/` in your IDE

# Happy Coding!
```
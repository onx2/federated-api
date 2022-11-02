# federated-api

This repo serves as an example of how to use cargo workspaces with docker to run microservices. However the services can each be run directly from their subdirectories provided they supply a top-level query, mutation, or subscription. The intended workflow for this repo is that all the services can be spun up at once and then the specific one you need to work on can be run in watch mode.

## Installation
- [Docker](https://www.docker.com/products/docker-desktop/)
- [Rust](https://www.rust-lang.org/tools/install)
  - This installs the toolchain manager, `rustup`, which manage `rustc`, `cargo`, and itself `rustup`.
- [Cargo Watch](https://github.com/watchexec/cargo-watch)
  - The standard approach is: `cargo install cargo-watch` once you've installed Rust.

## Getting Started
You can run all services at once using docker compose. Once all of the services are running, you can stop whichever you want from the Docker Desktop in favor of running it locally for a better developer experience.

- Run all microservices: `docker compose up -d;`
- Stop a service `<service_name>` of your choice, for example `user_service`
- `cd` into the service directory `<service_name>` from the repo's root
- Run `cargo watch -x run`

## Tools and technology
- Postgres
- Rust
  - async_graphl _(graphql server)_
  - poem _(http server)_
  - diesel _(ORM)_
- GraphQL (Apollo Federation)
- Docker

## Todo
- [x] Run a specific project with `cargo watch -x run` to leverage that dev experience.
- [ ] Apollo federation / graphql
- [ ] Logging / tracing
- [ ] CI / CD
- [ ] Testing
- [ ] Seeding

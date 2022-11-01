FROM lukemathwalker/cargo-chef:latest-rust-1.64 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG BIN
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin $BIN

# FROM gcr.io/distroless/cc AS runtime
FROM debian:buster-slim as runtime
RUN apt update
RUN apt install -y libpq-dev

ARG BIN
COPY --from=builder /app/target/release/$BIN /app
ENTRYPOINT [ "/app" ]

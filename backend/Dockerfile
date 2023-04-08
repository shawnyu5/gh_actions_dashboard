FROM rust:1.67.1-slim-buster AS build

WORKDIR /dashboard/
RUN apt-get update && apt-get -y install pkg-config openssl libssl-dev --no-install-recommends

COPY Cargo.toml .
# create a dummy file to force cargo to build the dependencies
RUN mkdir src \
   && echo "// dummy file" > src/lib.rs \
   && cargo build --lib --release

COPY src src
RUN cargo build --release

FROM rust:1.67.1-slim-buster

WORKDIR /dashboard/
COPY --from=build dashboard/target/release/gh_dashboard .
COPY rust-gh-action-dashboard.2023-04-01.private-key.pem .

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["./gh_dashboard"]


FROM rust:1.67.1-slim-buster AS build

WORKDIR /gh_action_dashboard
RUN apt-get update && apt-get -y install pkg-config openssl libssl-dev --no-install-recommends
RUN cargo install --locked trunk

COPY Cargo.toml .
COPY Cargo.lock .
COPY Trunk.toml .
# create a dummy file to force cargo to build the dependencies
RUN mkdir src \
   && echo "// dummy file" > src/lib.rs \
   && cargo build --lib --release

COPY src src
COPY index.html index.html
RUN rustup target add wasm32-unknown-unknown
RUN trunk build --release

EXPOSE 8080
CMD ["trunk", "serve", "--release"]

# FROM rust:1.67.1-slim-buster
# WORKDIR /gh_action_dashboard

# COPY --from=build /gh_action_dashboard/Cargo.toml .
# COPY --from=build /gh_action_dashboard/dist/ .

# RUN cargo install --locked trunk

FROM shawnyu5/rust-trunk:latest AS build

WORKDIR /gh_action_dashboard

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


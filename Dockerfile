FROM rust:1.74.0 as rust

RUN rustup component add clippy rustfmt
WORKDIR /app
COPY Cargo.toml Cargo.lock .rustfmt.toml ./
COPY src ./src
RUN cargo build --release
RUN cargo clippy
RUN cargo test
RUN cargo fmt -- --check

FROM fedora:39
WORKDIR /app
COPY --from=rust /app/target/release/dex_checksum_tools ./

ENV PATH=$PATH:/app

CMD 'dex_checksum_tools'
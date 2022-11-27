FROM rust:latest
RUN cargo new --bin WIK-DPS-TP02
WORKDIR /WIK-DPS-TP02
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release
RUN useradd -ms /bin/bash webservuser
USER webservuser
EXPOSE 8888
CMD ["./target/release/WIK-DPS-TP02"]
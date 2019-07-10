FROM rustlang/rust:nightly-slim AS builder

WORKDIR /opt/diario

RUN apt update && apt install libpq-dev -y

COPY Cargo.toml ./
COPY Cargo.lock ./

COPY src ./src

RUN cargo build --release
RUN cp ./target/release/diario .

COPY static ./static
COPY templates ./templates
COPY Rocket.toml ./

CMD ["./diario"]

FROM debian:buster

WORKDIR /opt/diario

RUN apt update && apt install curl libpq-dev pkg-config build-essential libssl-dev -y
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
ENV PATH "/root/.cargo/bin:${PATH}"

COPY Cargo.toml ./
COPY Cargo.lock ./

COPY src ./src

RUN cargo build
RUN cp ./target/debug/diario .

COPY static ./static
COPY templates ./templates
COPY Rocket.toml ./

CMD ["./diario"]

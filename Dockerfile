FROM rust:latest
MAINTAINER clma <claus.matzinger+kb@gmail.com>

RUN mkdir -p /app && git clone https://github.com/celaus/rust-hello-web /app \
    && cd /app \
    && cargo build --release  \
    && rm -Rf /app/src  /app/target/release/build /app/target/release/deps /app/target/release/examples/ /app/target/release/incremental/ /app/target/release/native
WORKDIR /app/target/release
CMD ["./hello-web"]

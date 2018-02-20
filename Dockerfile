FROM rust:latest
MAINTAINER clma <claus.matzinger+kb@gmail.com>

RUN mkdir -p /app && git clone https://github.com/celaus/rust-hello-web /app \
    && cd /app \
    && cargo build --release  \
    && rm -Rf /app/src /app/target/release/ /app/target/build /app/target/deps /app/target/examples/ /app/target/incremental/ /app/target/native
WORKDIR /app/target/release
CMD ["./hello-web"]

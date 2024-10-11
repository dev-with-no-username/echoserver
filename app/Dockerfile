FROM rust:1.69-slim-buster as builder

# below 2 commands is for build a static executable
# RUN apt-get update
# RUN apt-get install -y musl-dev

WORKDIR /wd

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# below 2 commands is for build a static executable
# RUN rustup target add x86_64-unknown-linux-musl
# RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN cargo build --release

FROM scratch
# FROM debian:buster-slim

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# below command is for build a static executable
# COPY --from=builder /wd/target/x86_64-unknown-linux-musl/release/app /
COPY --from=builder /wd/target/release/app /

# below commands are needed to have C libraries available in scratch container, because
# Rust executable needs them when built as a dynamic executable
COPY --from=builder /lib/x86_64-linux-gnu/libgcc_s.so.1 /lib/x86_64-linux-gnu/libgcc_s.so.1
COPY --from=builder /lib/x86_64-linux-gnu/libpthread.so.0 /lib/x86_64-linux-gnu/libpthread.so.0
COPY --from=builder /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/libm.so.6
COPY --from=builder /lib/x86_64-linux-gnu/libdl.so.2 /lib/x86_64-linux-gnu/libdl.so.2
COPY --from=builder /lib/x86_64-linux-gnu/libc.so.6 /lib/x86_64-linux-gnu/libc.so.6
COPY --from=builder /lib/x86_64-linux-gnu/librt.so.1 /lib/x86_64-linux-gnu/librt.so.1
COPY --from=builder /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2

CMD ["./app"]

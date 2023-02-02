FROM registry.suse.com/bci/rust:latest AS builder
WORKDIR /usr/src/fakemall
COPY ./ .
RUN cargo build --release

FROM registry.suse.com/bci/bci-base:latest
COPY --from=builder /usr/src/fakemall/target/release/fakemall /bin/fakemall
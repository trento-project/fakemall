FROM registry.suse.com/bci/rust:latest AS builder
WORKDIR /usr/src/fakesh
COPY ./ .
RUN cargo build --release

FROM registry.suse.com/bci/bci-base:latest
COPY --from=builder /usr/src/fakesh/target/release/fakesh /bin/fakesh
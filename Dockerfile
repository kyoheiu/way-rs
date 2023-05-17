FROM rust:1.69.0-slim-bookworm as builder
WORKDIR /home/way
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /home/way
COPY --from=builder /home/way/target/release/way . 
COPY static ./static
COPY templates ./templates
ENV RUST_LOG info
EXPOSE 9090
ENTRYPOINT [ "./way" ]

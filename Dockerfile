FROM rust:1.69.0-slim-bookworm as builder
WORKDIR /home/way
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /home/way/target/release/way . 
ENV RUST_LOG info
EXPOSE 9090
ENTRYPOINT [ "./way" ]

FROM node:20-slim as svelte-builder
WORKDIR /svelte
COPY ./svelte/package*.json ./
RUN npm install
COPY ./svelte .
RUN npm run build

FROM rust:1.69.0-slim-bookworm as builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /build/target/release/way . 
COPY --from=svelte-builder /svelte/dist/ ./static/
RUN mv ./static/font ./static/assets/
ENV RUST_LOG info
EXPOSE 9090
ENTRYPOINT [ "./way" ]

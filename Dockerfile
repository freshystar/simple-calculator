FROM rust:alpine AS builder

WORKDIR /usr/src/simple-calculator

COPY . .

RUN cargo build --release

FROM alpine:latest

COPY --from=builder /usr/src/simple-calculator/target/release/simple-calculator /

CMD ["./simple-calculator"] 
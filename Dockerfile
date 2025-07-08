FROM rust:1.88

COPY . .

RUN cargo build --release

CMD ["./target/release/ingrid"]

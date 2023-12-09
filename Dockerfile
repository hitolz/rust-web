ARG BASE_RUST=rust:1.71.0
FROM ${BASE_RUST}
COPY . .

RUN cargo build --release
 

CMD ["cargo", "run", "--release"]

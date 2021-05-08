FROM rust:1.52 as builder
WORKDIR /opt/taskmaster
COPY . .
RUN cargo install --bin taskmasterd --path .

FROM debian:buster-slim
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/taskmasterd /opt/taskmaster
CMD ["taskmasterd"]

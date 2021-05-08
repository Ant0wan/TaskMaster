FROM rust:1.52 as builder
WORKDIR /opt/taskmaster
COPY . .
RUN cargo install --bin taskmasterctl --path .

FROM debian:buster-slim
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/taskmasterctl /opt/taskmasterctl
CMD ["taskmasterctl"]

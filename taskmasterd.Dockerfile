# syntax=docker/dockerfile:1
FROM rust:1.67 AS builder
WORKDIR /
COPY . .
RUN cargo install --bin taskmasterd --path .

FROM ubuntu:22.04
#RUN apt-get update \
#	&& apt-get install -y <extra-runtime-dependencies> \
#	&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/taskmasterd /usr/local/bin/taskmasterd
CMD ["taskmasterd"]




#FROM ubuntu:22.04
#RUN apt-get update \
#	&& apt-get install -y supervisor=4.2.1-1ubuntu1 --no-install-recommends \
#	&& rm /var/lib/apt/lists/*
#RUN mkdir -p /var/log/supervisor
#COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
#COPY my_first_process my_first_process
#COPY my_second_process my_second_process
#CMD ["/usr/bin/supervisord"]

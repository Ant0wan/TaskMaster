# syntax=docker/dockerfile:1
FROM ubuntu:22.04
RUN apt-get update \
	&& apt-get install -y supervisor=4.2.1-1ubuntu1 --no-install-recommends \
	&& rm -rf /var/lib/apt/lists/*
RUN mkdir -p /var/log/supervisor
COPY config/supervisord.conf /etc/supervisor/supervisord.conf
COPY  /etc/supervisor/conf.d/supervisord.conf
CMD ["/usr/bin/supervisord"]

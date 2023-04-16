# syntax=docker/dockerfile:1
FROM ubuntu:22.04
RUN apt-get update \
	&& apt-get install -y supervisor=4.2.1-1ubuntu1 nginx --no-install-recommends \
	&& rm -rf /var/lib/apt/lists/*
RUN mkdir -p /var/log/supervisor
COPY config/supervisord.conf /etc/supervisor/supervisord.conf
COPY tools/ngnix.conf /etc/supervisor/conf.d/nginx.conf
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/supervisord.conf"]

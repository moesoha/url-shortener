FROM rust:1.26.0-slim-stretch AS stage-build
RUN mkdir -p /app/build
WORKDIR /app/build
COPY . .
# RUN sed -i -e "s/deb.debian.org/mirrors.tuna.tsinghua.edu.cn/g" /etc/apt/sources.list \
# 	&& sed -i -e "s/security.debian.org/mirrors.tuna.tsinghua.edu.cn/g" /etc/apt/sources.list
RUN apt-get update \
	&& apt-get install -y build-essential musl-tools
RUN rustup default nightly-2018-05-30 \
	&& rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.7
RUN mkdir -p /app/bin
COPY --from=stage-build /app/build/target/x86_64-unknown-linux-musl/release/url-shortener /app/bin/
WORKDIR /app
CMD ["./bin/url-shortener"]

FROM ghcr.io/rust-lang/rust:nightly-slim
RUN apt-get update \
    && apt-get upgrade \
    && apt-get install -y cmake pkg-config libssl-dev time
RUN cargo install cargo-x


COPY . /app
WORKDIR /app
RUN time cargo build

EXPOSE 3000
CMD ["cargo", "run"]

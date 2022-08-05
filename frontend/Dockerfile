FROM ghcr.io/rust-lang/rust:nightly-slim
RUN apt-get update \
    && apt-get upgrade \
    && apt-get install -y cmake pkg-config libssl-dev wget inotify-tools time

RUN cargo install trunk@0.16.0 cargo-x
RUN cargo install --locked wasm-bindgen-cli
RUN wget  https://nodejs.org/dist/v18.7.0/node-v18.7.0-linux-x64.tar.xz -O node.tar.xz
RUN tar -xvf node.tar.xz
RUN cp -rf node-v18.7.0-linux-x64/* /usr/
RUN chmod +rx /usr/bin/npx
RUN rustup target add wasm32-unknown-unknown


COPY . /app
WORKDIR /app
RUN time cargo x build

EXPOSE 8080

CMD ["cargo", "x", "dev"]
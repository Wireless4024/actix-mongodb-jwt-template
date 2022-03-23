FROM rustlang/rust:nightly-slim
# latest nightly version of rust

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install -f --path .
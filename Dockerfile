FROM rust:1.31

WORKDIR /usr/src/maze
COPY . .

RUN cargo build --release

CMD ["maze"]
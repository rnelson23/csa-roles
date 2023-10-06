FROM rust

WORKDIR /usr/src/app

COPY . .

RUN cargo build

CMD ["cargo", "run"]

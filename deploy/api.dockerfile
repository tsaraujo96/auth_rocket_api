FROM rust:1.72.1

ENV ROCKET_ADDRESS=127.0.0.1
ENV ROCKET_PORT=8000

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]
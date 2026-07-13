FROM rust:1.94-slim AS build
WORKDIR /src
COPY . .
RUN cargo build --release --bin loom-server --bin loom-indexer

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 loom
COPY --from=build /src/target/release/loom-server /usr/local/bin/loom-server
COPY --from=build /src/target/release/loom-indexer /usr/local/bin/loom-indexer
USER loom
CMD ["loom-server"]

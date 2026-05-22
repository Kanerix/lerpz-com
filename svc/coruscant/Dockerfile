FROM rust:slim-bookworm AS chef

RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y --no-install-recommends \
    build-essential pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

RUN --mount=type=cache,id=cargo,target=/root/.cargo/registry \
    cargo install cargo-chef


FROM chef AS planner
WORKDIR /chef

COPY Cargo.toml Cargo.lock ./
COPY svc/coruscant ./svc/coruscant

RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
WORKDIR /build

COPY --from=planner /chef/recipe.json ./recipe.json
RUN --mount=type=cache,id=cargo,target=/root/.cargo/registry \
    --mount=type=cache,id=deps,target=/build/target \
    cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.lock ./
COPY svc/coruscant ./svc/coruscant
COPY crates ./crates

ENV SQLX_OFFLINE=true

RUN --mount=type=cache,id=cargo,target=/root/.cargo/registry \
    --mount=type=cache,id=target,target=/build/target \
    cargo build --release --bin coruscant && \
    cp /build/target/release/coruscant /build/server


FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    addgroup --system --gid 1001 app && \
    adduser --system --uid 1001 --ingroup app lerpz

COPY --chown=lerpz:app --from=builder /build/server ./

USER lerpz

EXPOSE 3001

ENTRYPOINT ["/app/server"]

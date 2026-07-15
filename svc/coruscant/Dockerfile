FROM oven/bun:1-slim AS base


FROM base AS builder
WORKDIR /app

COPY package.json bun.lock ./
COPY svc/coruscant/package.json ./svc/coruscant/
COPY packages/ui/package.json ./packages/ui/
COPY packages/biome-config/package.json ./packages/biome-config/
COPY packages/typescript-config/package.json ./packages/typescript-config/

RUN --mount=type=cache,id=bun,target=/root/.bun/install/cache \
    bun install --frozen-lockfile

COPY svc/coruscant ./svc/coruscant
COPY packages/ui ./packages/ui
COPY packages/biome-config ./packages/biome-config
COPY packages/typescript-config ./packages/typescript-config

RUN --mount=type=cache,id=vite,target=/app/svc/coruscant/.vite-cache \
    cd svc/coruscant && bun run build


FROM base AS prod-deps
WORKDIR /app

COPY package.json bun.lock ./
COPY svc/coruscant/package.json ./svc/coruscant/
COPY packages/ui/package.json ./packages/ui/
COPY packages/biome-config/package.json ./packages/biome-config/
COPY packages/typescript-config/package.json ./packages/typescript-config/

RUN --mount=type=cache,id=bun,target=/root/.bun/install/cache \
    bun install --production --frozen-lockfile


FROM base AS runner
WORKDIR /app/svc/coruscant

ENV NODE_ENV=production \
    PORT=3000

RUN groupadd --system --gid 1001 app && \
    useradd --system --no-log-init --uid 1001 --gid app lerpz

COPY --from=prod-deps --chown=lerpz:app /app /app

COPY --from=builder --chown=lerpz:app /app/svc/coruscant/build ./build

USER lerpz

EXPOSE 3000

CMD ["bun", "build/index.js"]

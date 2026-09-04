FROM oven/bun:1-slim AS base


FROM base AS builder
WORKDIR /app

COPY package.json bun.lock ./
COPY svc/www/package.json ./svc/www/
COPY packages/ui/package.json ./packages/ui/
COPY packages/biome-config/package.json ./packages/biome-config/
COPY packages/typescript-config/package.json ./packages/typescript-config/

RUN --mount=type=cache,id=bun,target=/root/.bun/install/cache \
    bun install --frozen-lockfile

COPY svc/www ./svc/www
COPY packages/ui ./packages/ui
COPY packages/biome-config ./packages/biome-config
COPY packages/typescript-config ./packages/typescript-config

RUN --mount=type=cache,id=vite,target=/app/svc/www/.vite-cache \
    cd svc/www && bun run build


FROM nginxinc/nginx-unprivileged:1-alpine AS runner

COPY svc/www/nginx.conf /etc/nginx/conf.d/default.conf

COPY --from=builder /app/svc/www/build /usr/share/nginx/html

EXPOSE 3000

FROM node:22-slim AS base

RUN corepack enable pnpm


FROM base AS deps
WORKDIR /app


COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
COPY svc/portal/package.json ./svc/portal/

RUN --mount=type=cache,id=pnpm,target=/root/.local/share/pnpm/store \
    pnpm install --frozen-lockfile


FROM base AS builder
WORKDIR /app

COPY --from=deps /app/node_modules ./node_modules
COPY --from=deps /app/svc/portal/node_modules ./svc/portal/node_modules

COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
COPY svc/portal ./svc/portal

ENV NEXT_TELEMETRY_DISABLED=1

RUN --mount=type=cache,id=nextjs,target=/app/svc/portal/.next/cache \
    --mount=type=cache,id=turbo,target=/app/.turbo \
    pnpm --filter portal build


FROM base AS runner
WORKDIR /app

ENV NODE_ENV=production \
    NEXT_TELEMETRY_DISABLED=1 \
    PORT=3000

RUN addgroup --system --gid 1001 app && \
    adduser --system --uid 1001 --ingroup app lerpz

COPY --from=builder --chown=lerpz:app /app/svc/portal/.next/standalone ./
COPY --from=builder --chown=lerpz:app /app/svc/portal/.next/static ./svc/portal/.next/static
COPY --from=builder --chown=lerpz:app /app/svc/portal/public ./svc/portal/public

USER lerpz

EXPOSE 3000

CMD ["node", "svc/portal/server.js"]

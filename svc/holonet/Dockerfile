FROM node:22-slim AS base

RUN corepack enable pnpm


FROM base AS deps
WORKDIR /app


COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
COPY svc/holonet/package.json ./svc/holonet/
COPY packages/ui/package.json ./packages/ui/
COPY packages/typescript-config/package.json ./packages/typescript-config/

RUN --mount=type=cache,id=pnpm,target=/root/.local/share/pnpm/store \
    pnpm install --frozen-lockfile


FROM base AS builder
WORKDIR /app

COPY --from=deps /app/node_modules ./node_modules

COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
COPY svc/holonet ./svc/holonet
COPY packages/ui ./packages/ui
COPY packages/typescript-config ./packages/typescript-config

ENV NEXT_TELEMETRY_DISABLED=1 \
    SKIP_ENV_VALIDATION=1

ARG NEXT_PUBLIC_API_URL
ARG NEXT_PUBLIC_ENTRA_ID_TENANT_ID
ARG NEXT_PUBLIC_ENTRA_ID_CLIENT_ID
ARG NEXT_PUBLIC_ENTRA_ID_SCOPE
ARG NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI
ARG NEXT_PUBLIC_ENTRA_ID_LOGOUT_URI

ENV NEXT_PUBLIC_API_URL=$NEXT_PUBLIC_API_URL \
    NEXT_PUBLIC_ENTRA_ID_TENANT_ID=$NEXT_PUBLIC_ENTRA_ID_TENANT_ID \
    NEXT_PUBLIC_ENTRA_ID_CLIENT_ID=$NEXT_PUBLIC_ENTRA_ID_CLIENT_ID \
    NEXT_PUBLIC_ENTRA_ID_SCOPE=$NEXT_PUBLIC_ENTRA_ID_SCOPE \
    NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI=$NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI \
    NEXT_PUBLIC_ENTRA_ID_LOGOUT_URI=$NEXT_PUBLIC_ENTRA_ID_LOGOUT_URI

RUN --mount=type=cache,id=nextjs,target=/app/svc/holonet/.next/cache \
    --mount=type=cache,id=turbo,target=/app/.turbo \
    pnpm --filter holonet build


FROM base AS runner
WORKDIR /app

ENV NODE_ENV=production \
    NEXT_TELEMETRY_DISABLED=1 \
    PORT=3000

RUN addgroup --system --gid 1001 app && \
    adduser --system --uid 1001 --ingroup app lerpz

COPY --from=builder --chown=lerpz:app /app/svc/holonet/.next/standalone ./
COPY --from=builder --chown=lerpz:app /app/svc/holonet/.next/static ./svc/holonet/.next/static
COPY --from=builder --chown=lerpz:app /app/svc/holonet/public ./svc/holonet/public

USER lerpz

EXPOSE 3000

CMD ["node", "svc/holonet/server.js"]

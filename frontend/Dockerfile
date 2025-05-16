# ── Builder Stage ────────────────────────────────────────────────────────────
FROM oven/bun:1.2.13-alpine AS builder

# 1) Alpine needs CA certs so Bun can fetch remote plugins over HTTPS
RUN apk add --no-cache ca-certificates

WORKDIR /app

# 2) Copy lockfiles first and install deps (caches nicely)
COPY package.json yarn.lock bun.lockb ./
RUN bun install --frozen-lockfile

# 3) Copy the rest of your source (including any inlang.config.js or .env)
COPY . .

# 4) Generate SvelteKit’s .svelte-kit (so tsconfig.json extends resolve)
RUN bun exec svelte-kit sync

# 5) Build your app
RUN bun run build


# ── Runner Stage ────────────────────────────────────────────────────────────
FROM oven/bun:1.2.13-alpine AS runner

WORKDIR /app
ENV NODE_ENV=production

# 6) Copy just the built output + manifest
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./
COPY --from=builder /app/bun.lockb ./

# 7) Install production-only deps
RUN bun install --production --frozen-lockfile

# 8) Expose and run
EXPOSE 3000
CMD ["bun", "run", "build/index.js"]

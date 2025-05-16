# ── Builder Stage ────────────────────────────────────────────────────────────
FROM oven/bun:1.2.13-alpine AS builder

# 1) Alpine needs CA certs so Bun can fetch remote plugins over HTTPS
RUN apk add --no-cache ca-certificates

WORKDIR /app

# 2) Copy just your manifests; bun will generate bun.lockb
COPY package.json yarn.lock ./

# 3) Install all deps (dev + prod), producing bun.lockb
RUN bun install --frozen-lockfile

# 4) Copy the rest of your source (including any inlang.config.js, .env, etc)
COPY . .

# 5) Generate SvelteKit internals (.svelte-kit) via Bun’s npx-alias
RUN bun x svelte-kit sync

# 6) Build your app
RUN bun run build


# ── Runner Stage ────────────────────────────────────────────────────────────
FROM oven/bun:1.2.13-alpine AS runner

WORKDIR /app
ENV NODE_ENV=production

# 7) Copy built output + lockfile + manifest
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./
COPY --from=builder /app/bun.lockb ./

# 8) Install only production deps
RUN bun install --production --frozen-lockfile

# 9) Expose port & run
EXPOSE 3000
CMD ["bun", "run", "build/index.js"]

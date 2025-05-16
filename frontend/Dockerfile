# ── Builder Stage ────────────────────────────────────────────────────────────
FROM oven/bun:1.2.13-alpine AS builder

# Alpine needs CA certs for HTTPS imports
RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy just the manifests; bun will generate bun.lockb here
COPY package.json yarn.lock ./

# Install all deps (dev + prod), produces bun.lockb
RUN bun install --frozen-lockfile

# Copy the rest of your source (including inlang.config.js, .env, etc)
COPY . .

# Generate SvelteKit internals (.svelte-kit)
RUN bun exec svelte-kit sync

# Build your app
RUN bun run build


# ── Runner Stage ────────────────────────────────────────────────────────────
FROM oven/bun:1.2.13-alpine AS runner

WORKDIR /app
ENV NODE_ENV=production

# Copy built output + lockfile + manifest
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./
COPY --from=builder /app/bun.lockb ./

# Install only production deps
RUN bun install --production --frozen-lockfile

EXPOSE 3000
CMD ["bun", "run", "build/index.js"]

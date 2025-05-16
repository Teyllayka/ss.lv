FROM oven/bun:1.1 AS builder
WORKDIR /app

# Copy package files first for better caching
COPY package.json bun.lockb* ./
RUN bun install --frozen-lockfile

# Copy the rest of the application
COPY . .

# Add SvelteKit initialization step before building
RUN bun run vite sync || bun run svelte-kit sync
# Then build
RUN bun run build

FROM oven/bun:1.1 AS runner
WORKDIR /app

ENV NODE_ENV=production

# Copy only what's needed to run the application
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./package.json

# Install only production dependencies
RUN bun install --production --frozen-lockfile

EXPOSE 3000
CMD ["bun", "build/index.js"]

FROM oven/bun:1.1 AS builder
WORKDIR /app

# Copy package files first for better caching
COPY package.json bun.lockb* ./
# Force Node.js compatibility mode for packages that might not work well with Bun
ENV NODE_OPTIONS="--no-warnings"
ENV BUN_INSTALL_CACHE=/app/.bun_cache

# Install dependencies
RUN bun install --frozen-lockfile

# Copy the rest of the application
COPY . .

# Fix for SvelteKit tsconfig issue - ensure .svelte-kit directory exists before build
RUN mkdir -p .svelte-kit
RUN touch .svelte-kit/tsconfig.json

# Run the build with NODE_ENV set to production to avoid certain dev-only behaviors
RUN NODE_ENV=production bun run build    # outputs to ./build

FROM oven/bun:1.1 AS runner
WORKDIR /app

ENV NODE_ENV=production

# Copy only necessary files from builder
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./package.json

# Install production dependencies only
RUN bun install --production --frozen-lockfile

EXPOSE 3000
CMD ["bun", "build/index.js"]
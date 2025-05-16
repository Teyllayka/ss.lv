FROM oven/bun:1.1 AS builder
WORKDIR /app

COPY package.json bun.lockb* ./
RUN bun install --frozen-lockfile

COPY . .
RUN bun run build    # outputs to ./build

FROM oven/bun:1.1 AS runner
WORKDIR /app

ENV NODE_ENV=production

COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./package.json

RUN bun install --production --frozen-lockfile

EXPOSE 3000
CMD ["bun", "build/index.js"]
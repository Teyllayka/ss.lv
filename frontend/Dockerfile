FROM oven/bun:1.2.13-alpine AS builder
WORKDIR /app

COPY package.json yarn.lock ./
RUN bun install --frozen-lockfile                                         

COPY . .
RUN bun run build                                                        

FROM oven/bun:1.2.13-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production

COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./
COPY --from=builder /app/bun.lockb ./

RUN bun install --production --frozen-lockfile                          

EXPOSE 3000
CMD ["bun", "run", "build/index.js"]

FROM node:22-alpine AS builder
WORKDIR /app

COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

COPY . .
RUN yarn build    # outputs to ./build

FROM node:22-alpine AS runner
WORKDIR /app

ENV NODE_ENV=production

COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./package.json

RUN yarn install --production --frozen-lockfile

EXPOSE 3000
CMD ["node", "--max-old-space-size=2048", "build/index.js"]

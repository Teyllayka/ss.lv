FROM node:24-slim AS builder
WORKDIR /app

COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

COPY . .
RUN yarn build    # outputs to ./build

FROM node:24-slim AS runner
WORKDIR /app

ENV NODE_ENV=production

ENV NODE_OPTIONS=--max-old-space-size=4096

COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./package.json

RUN yarn install --production --frozen-lockfile

EXPOSE 3000
CMD ["node", "build/index.js"]

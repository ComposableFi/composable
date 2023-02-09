FROM node:16-alpine AS node
CMD ["echo", "'initializing graphql-server build main Dockerfile'"]
FROM node AS node-with-gyp
RUN apk --no-cache add g++ make python3

FROM node-with-gyp AS builder
WORKDIR /squid
COPY package.json .
COPY package-lock.json .
RUN npm ci
COPY tsconfig.json .
COPY src src
COPY db db
COPY dbInit.sql dbInit.sql
RUN npm run build

FROM node-with-gyp AS deps
WORKDIR /squid
COPY package.json .
COPY package-lock.json .
RUN npm ci

FROM node AS squid
WORKDIR /squid
COPY --from=deps /squid/package.json .
COPY --from=deps /squid/package-lock.json .
COPY --from=deps /squid/node_modules node_modules
COPY --from=builder /squid/lib lib
COPY --from=builder /squid/db db
COPY --from=builder /squid/dbInit.sql dbInit.sql
COPY scripts scripts
COPY schema.graphql .

ENTRYPOINT ["/squid/scripts/init.sh"]

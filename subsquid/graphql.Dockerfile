FROM node:16-alpine AS node
CMD ["echo", "'initializing graphql-server build graphql.Dockerfile'"]
FROM node AS node-with-gyp
RUN apk add g++ make python3

FROM node-with-gyp AS builder
WORKDIR /squid
ADD package.json .
ADD package-lock.json .
RUN npm ci
ADD tsconfig.json .
ADD src src
RUN npm run build

FROM node-with-gyp AS deps
WORKDIR /squid
ADD package.json .
ADD package-lock.json .
RUN npm ci

FROM node AS squid
WORKDIR /squid
COPY --from=deps /squid/package.json .
COPY --from=deps /squid/package-lock.json .
COPY --from=deps /squid/node_modules node_modules
COPY --from=builder /squid/lib lib
ADD schema.graphql .

FROM squid AS query-node

EXPOSE 4350

ADD ./scripts/init.sh /init.sh
ENTRYPOINT ["sh", "/init.sh"]

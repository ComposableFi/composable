FROM node:16

COPY * /build/
WORKDIR /build

RUN yarn build

ENTRYPOINT ["yarn", "start"]

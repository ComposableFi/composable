FROM node:19

WORKDIR /upgrader
RUN git clone https://github.com/maxsam4/fork-off-substrate.git

WORKDIR /upgrader/fork-off-substrate
RUN npm i && mkdir data

WORKDIR /upgrader
COPY *.json ./
RUN npm install
COPY . .
ENTRYPOINT [ "node", "upgrader.mjs" ]
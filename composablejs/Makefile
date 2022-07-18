
install:
	yarn install

build:
	yarn build

build-all:
	npx lerna run build

build-types:
	cd packages/types && yarn gen && yarn build

build-utils:
	cd packages/utils && yarn build

build-oracle_setup:
	cd packages/oracle_setup && yarn build

lint:
	yarn lint

oracle_setup:
	cd packages/oracle_setup && yarn install && yarn start

process: migrate
	@node -r dotenv/config lib/processor.js

build:
	@npm run build

build-processor-image:
	@docker build . -t squid-processor

build-images: build-processor-image

serve:
	@npx squid-graphql-server --subscriptions

rebuild-graphql:
	make build-images
	docker-compose stop graphql-server
	docker-compose rm --force graphql-server
	docker-compose create graphql-server
	docker-compose start graphql-server
	docker-compose start processor


rebuild-graphql-local:
	make build-images
	docker-compose -f docker-compose-dev.yml stop graphql-server
	docker-compose -f docker-compose-dev.yml rm --force graphql-server
	docker-compose -f docker-compose-dev.yml create graphql-server
	docker-compose -f docker-compose-dev.yml start graphql-server

start:
	make down
	docker volume rm -f $$(docker volume ls)
	make build-images
	make up


start-local:
	make down-local
	docker volume rm -f $$(docker volume ls)
	make build-images
	make up-local


migrate:
	@npx squid-typeorm-migration apply
	npm run build && node lib/helperInit.js


codegen:
	@npx squid-typeorm-codegen


typegen:
	@npx squid-substrate-typegen typegen.json


up:
	@docker-compose up -d


up-local:
	@docker-compose -f docker-compose-dev.yml up -d


down:
	@docker-compose down


down-local:
	@docker-compose -f docker-compose-dev.yml down


explore:
	npx squid-substrate-metadata-explorer --chain wss://picasso-rpc-lb.composablenodes.tech --out picassoVersions.jsonl


sqlinit:
	npm run build && node lib/helperInit.js



.PHONY: build serve process migrate codegen typegen up down explore rebuild

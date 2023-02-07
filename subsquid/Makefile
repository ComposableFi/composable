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

restart:
	make down
	docker volume rm -f $(docker volume ls)
	make build-images
	make up


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
	docker compose up db ingest gateway explorer squid-db -d


down:
	@docker-compose down


explore:
	npx squid-substrate-metadata-explorer --chain wss://picasso-rpc-lb.composablenodes.tech --out picassoVersions.jsonl


sqlinit:
	npm run build && node lib/helperInit.js



.PHONY: build serve process migrate codegen typegen up down explore rebuild

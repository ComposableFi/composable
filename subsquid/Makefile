process: migrate
	@node -r dotenv/config lib/processor.js

build:
	@npm run build

build-processor-image:
	@docker build . -t squid-processor

build-query-node-image:
	@docker build . -t graphql-server -f graphql.Dockerfile

build-images: build-processor-image build-query-node-image

serve:
	@npx squid-graphql-server --subscriptions


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
	rm -f daliDevVersions.jsonl
	npx squid-substrate-metadata-explorer --chain ws://127.0.0.1:9988 --out daliDevVersions.jsonl


sqlinit:
	npm run build && node lib/helperInit.js


rebuild:
	make down
	rm -f -r lib
	rm -rf db/migrations/*.js
	make up
	npx squid-typeorm-migration generate
	make migrate


.PHONY: build serve process migrate codegen typegen up down explore rebuild

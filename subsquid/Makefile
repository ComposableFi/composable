process: migrate
	@node -r dotenv/config lib/processor.js

build:
	@npm run build

build-processor-image:
	@docker build . --target processor -t squid-processor

build-query-node-image:
	@docker build . --target query-node -t query-node

build-images: build-processor-image build-query-node-image

serve:
	@npx squid-graphql-server --subscriptions


migrate:
	@npx squid-typeorm-migration apply


codegen:
	@npx squid-typeorm-codegen


typegen:
	@npx squid-substrate-typegen typegen.json


up:
	@docker-compose up -d


down:
	@docker-compose down


explore:
	rm -f daliDevVersions.jsonl
	npx squid-substrate-metadata-explorer --chain ws://127.0.0.1:9988 --out daliDevVersions.jsonl


rebuild:
	make down
	rm -f -r lib
	rm -rf db/migrations/*.js
	make up
	make codegen
	make build
	npx squid-typeorm-migration generate
	make migrate


.PHONY: build serve process migrate codegen typegen up down explore rebuild

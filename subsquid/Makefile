process: migrate
	@node -r dotenv/config lib/processor.js


serve:
	@npx squid-graphql-server


migrate:
	@npx sqd db:migrate


migration:
	@npx sqd db:create-migration Data


build:
	@npm run build


codegen:
	@npx sqd codegen


typegen: kusamaVersions.json
	@npx squid-substrate-typegen typegen.json


kusamaVersions.json:
	@make explore


explore:
	@npx squid-substrate-metadata-explorer \
		--chain wss://kusama-rpc.polkadot.io \
		--archive https://kusama.indexer.gc.subsquid.io/v4/graphql \
		--out kusamaVersions.json


up:
	@docker-compose up -d


down:
	@docker-compose down

rebuild:
	rm -f -r lib
	rm -f -r db/migrations
	npx sqd db drop
	npx sqd db create
	npm run build
	npx sqd codegen
	npx sqd db create-migration Init
	npx sqd db migrate


.PHONY: process serve start codegen migration migrate up down

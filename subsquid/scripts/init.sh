cd /squid
npx squid-typeorm-migration generate
npx squid-typeorm-migration apply
npx @subsquid/graphql-server

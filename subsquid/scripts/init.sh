cd /app
echo "Initializing subsquid processor graphql server"
npm ci
npm run build
rm -rf db/migrations/*
npx squid-typeorm-migration create
npx squid-typeorm-migration apply
npx @subsquid/graphql-server

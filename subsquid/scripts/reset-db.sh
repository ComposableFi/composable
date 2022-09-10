set -e
npm run typegen
npx squid-typeorm-codegen
npm run build
#rm -rf db/migrations/*.js
npx squid-typeorm-migration
npx squid-typeorm-migration create
#npx sqd db create-migration Init
npx squid-typeorm-migration migrate

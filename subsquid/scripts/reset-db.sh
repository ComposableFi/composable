set -e
npx squid-typeorm-codegen
npm run build
#rm -rf db/migrations/*.js
npx squid-typeorm-migration
npx squid-typeorm-migration generate
#npx sqd db create-migration Init
npx squid-typeorm-migration migrate

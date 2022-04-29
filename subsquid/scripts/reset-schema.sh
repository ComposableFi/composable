set -e
npx sqd codegen
npm run build
./reset-db.sh
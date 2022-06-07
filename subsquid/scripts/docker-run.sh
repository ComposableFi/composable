set -e
docker build . --target processor -t squid-processor
PORT=${1:-23798}
echo "Database port: ${PORT}"
# make sure the port matches .env. 
# For Linux, add --add-host=host.docker.internal:host-gateway
docker run --rm -e DB_HOST=host.docker.internal -e DB_PORT=$PORT squid-processor
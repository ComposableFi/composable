FILENAME=$(date -d yesterday +'%m-%d-%Y').zip
GS_BUCKET="$RUNTIME-data-store"
gsutil cp gs://$GS_BUCKET/"$FILENAME" .
echo $(sha1sum "$FILENAME") > hash.txt
unzip -o "$FILENAME" -d /tmp/db
cp -a /tmp/db/. $(pwd)/code/integration-tests/local-integration-tests/simnode-data/

#!/bin/sh
DATE=$(date +'%m-%d-%Y')
FILENAME=composable-$DATE.zip
BACKUP_DIR="/var/lib/composable-data/chains"
GS_BUCKET="composable-picasso-data-sync"
zip -r $FILENAME $BACKUP_DIR
gsutil mv $FILENAME gs://$GS_BUCKET/

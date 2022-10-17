#!/usr/bin/env bash
set -e
IMAGE="composablefi/composable:latest"
CONTAINER_ID=$(docker ps | grep $IMAGE| awk '{print $1}')
AUTO_UPDATE=1
# docker pull $IMAGE
if [ "$AUTO_UPDATE" = 1 ]; then
    for im in $CONTAINER_ID       
    do
        LATEST=`docker inspect --format "{{.Id}}" $IMAGE`
        RUNNING=`docker inspect --format "{{.Image}}" $im`
        NAME=`docker inspect --format '{{.Name}}' $im | sed "s/\///g"`
        echo "Latest:" $LATEST
        echo "Running:" $RUNNING
        if [ "$RUNNING" != "$LATEST" ];then
            echo "upgrading $NAME"
            docker pull $IMAGE
            docker stop $im
            docker rm -f $NAME
            # Add the command to run your container here
            # docker run $NAME 
        else
            echo "$NAME up to date"
        fi
    done
else
    echo "No latest update to upstream"
fi

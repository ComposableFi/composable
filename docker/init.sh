#!/usr/bin/env bash
set -e
IMAGE="nginx"
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
            #ToDO reverse engineer a running container to get the command used to run a container and use it to start the container again
            # docker run $NAME 
        else
            echo "$NAME up to date"
        fi
    done
else
    echo "No latest update to upstrean"
fi


## Logic 2 ##
# Pull and install git-repo-watcher, 10 sec

#bash ./git-repo-watcher/git-repo-watcher -d /composable -i 30  > "repo-watcher.log" 

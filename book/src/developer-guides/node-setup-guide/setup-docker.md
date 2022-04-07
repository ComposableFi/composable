## Setup docker and docker-compose

```bash
sudo apt install apt-transport-https ca-certificates curl gnupg-agent software-properties-common 
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
sudo apt install docker-ce docker-ce-cli containerd.io
```

Optional steps


```bash

sudo apt-mark hold docker-ce # prevent the Docker package from being updated, so no sudden updates and process interuption
sudo usermod -aG docker $USER # adds docker to sudo group so there's no need to run it from root
```

Setup docker compose 

```bash

sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose

sudo chmod +x /usr/local/bin/docker-compose
```

Check docker installation 
```bash

sudo systemctl status docker
docker container run hello-world
```

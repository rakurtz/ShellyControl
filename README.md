# ShellyControl
ShellyControl is a small hub to control your Shelly LEDs via a webinterface. It consists of 
- a rust backend
- a react frontend
served in your local network via Docker containers (leveraging Treafik as a fine proxy).



## Steps to run the ShellyControl via Docker
### install docker
take a look at the docker.io website for installation guides [[https://docs.docker.com/get-docker/]]

### configure docker to restart containers after reboot (linux):
```
sudo systemctl enable docker
```

### adjust compose.yaml to your needs
You may want to adjust the port, the react frontend is exposed to your local network. Default is `80`.


### run docker compose
```
docker compose up -d
```

### visit website
open "http://your-host" 
(maybe adjust the port to what you have specified in the traefik service)
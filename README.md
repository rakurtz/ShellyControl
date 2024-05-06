# ShellyControl
ShellyControl is a small hub to control your Shelly LEDs via a webinterface. It consists of 
- a rust backend
- a react frontend
served in your local network via Docker containers (leveraging Treafik as a fine proxy).



## Steps to run the ShellyControl via Docker
### adjust compose.yaml to your HOST
in `./traefik-proxy/config.yaml` adjust thes lines to the domain you want the Shelly Control to be accessible from 
`"traefik.http.routers.frontend.rule=Host(`INSERT_YOUR_HOST_HERE`) && PathPrefix(`/`)"` 
and
`"traefik.http.routers.backend.rule=Host(`INSERT_YOUR_HOST_HERE`) && PathPrefix(`/api`)"` 

You may also adjust the port, the react frontend is exposed to your local network. Default is `80`.


### run docker compose
```
docker network create web 
docker compose up -d --build
```

### visit website
open "http://your-host" 
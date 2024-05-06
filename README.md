# Steps to run the shelly fullstack


## adjust compose.yaml to your HOST
in `./traefik-proxy/config.yaml` adjust thes lines to the domain you want the Shelly Control to be accessible from 
`"traefik.http.routers.frontend.rule=Host(`INSERT_YOUR_HOST_HERE`) && PathPrefix(`/`)"` 
and
`"traefik.http.routers.backend.rule=Host(`INSERT_YOUR_HOST_HERE`) && PathPrefix(`/api`)"` 


## run docker compose
```
docker network create web 
docker compose up -d --build
```

## visit website
visit "http://your-host"
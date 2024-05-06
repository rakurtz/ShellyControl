# Todo
## data structure
- add id field to Member in lamps.rs and substitute "name" by "id" in the whole logic of handling shellies
- switch back from yaml to json

## routes
- only use 
    - /set/{lamp_id}/{command}/{brightness} with command = on, off or toggle
    - /get/{lamp_id}
    - /get_all
    - /all_off
- inside /set/{lamp_id}/{command}/{brightness} decide what actions to take (brightness?)

## new features
- implement a route for setting new lamps
- implement a local shelly discovery logic on demand and add newly discovered shellies to local state
- implement continously requesting state of all lamps and (like once every minute) 
- implement refresh via websocket 
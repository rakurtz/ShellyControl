#!/bin/bash

# chromium kioskmode
tmux new-session -d -s chromium 'DISPLAY=:0 chromium-browser --noerrdialogs --disable-translate --kiosk --incognito http://localhost'

## attach to session 
tmux attach -t chromium
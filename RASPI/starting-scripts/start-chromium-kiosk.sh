#!/bin/bash

# chromium kioskmode
tmux new-session -d -s chromium 'DISPLAY=:0 chromium-browser --noerrdialogs --disable-translate --kiosk --incognito http://localhost'


## use tmux attach -t chromium to attach to the tmux session










#!/bin/bash

### display & screensaver
export DISPLAY=:0
sudo sh -c 'echo 30 > /sys/class/backlight/10-0045/device/backlight/10-0045/brightness'
xset s 60       # in seconds (motion sensor kind of overrides this anyway. 
                #Should be longer than the interval coming from sensor


### tmux layout
tmux new-session -d -s shelly-servers
tmux split-window -v
tmux split-window -v -t 0
tmux select-layout even-vertical

# Execute commands in each pane
tmux send-keys -t 0 'cd ~/shelly-remote/rust/; ./shelly-actix-api' C-m
tmux send-keys -t 1 'cd ~/shelly-remote/react; sudo serve -s build -l 80' C-m
tmux send-keys -t 2 'DISPLAY=:0 ~/shelly-remote/python-scripts/motion-sensor.py' C-m

# Attach to the tmux session to view the output
tmux attach-session -t shelly-servers

# prevent terminal from closing
bash


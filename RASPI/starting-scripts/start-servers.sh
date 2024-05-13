#!/bin/bash

### display & screensaver
export DISPLAY=:0
sudo sh -c 'echo 30 > /sys/class/backlight/10-0045/device/backlight/10-0045/brightness'
xset s 60       # in seconds (motion sensor kind of overrides this anyway. 
                #Should be longer than the interval coming from sensor


### tmux layout
tmux new-session -d -s motion-sensor 'DISPLAY=:0 ~/shellycontrol/motion-sensor.py'

# attach to the tmux session to view the output
tmux attach-session -t motion-sensor

# prevent terminal from closing
bash
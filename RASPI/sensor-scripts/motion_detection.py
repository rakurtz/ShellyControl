import os
import requests
import datetime

from gpiozero import MotionSensor
from signal import pause

## Globals
API_URL = "http://localhost:3000/movement"

def print_motion():
    dt = datetime.datetime.today().replace(microsecond=0)
    print('{} - Detected motion!'.format(dt))

def display_on():
    os.system('DISPLAY=:0 xset s reset')

def call_to_api():
    response = requests.get(API_URL)
    print(response)

def motion():
    print_motion()
    display_on()
    #call_to_api()

def no_motion():
    dt = datetime.datetime.today().replace(microsecond=0)
    print('{} - "No motion" invoked...'.format(dt))
    os.system('DISPLAY=:0 xset s activate')

def main():
    print("motion sensor started...")
    print("on errors: start with DISPLAY=:0...")
    print("API-Endpoint is: {}".format(API_URL))
    pir = MotionSensor(4)

    pir.when_motion = motion
    pir.when_no_motion =  no_motion

    ## keeps the script alive
    pause()

if __name__ == "__main__":
    main()  


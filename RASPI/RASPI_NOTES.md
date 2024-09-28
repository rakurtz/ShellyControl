# Notes on configuring Raspi

## Setup

* ~~Raspi 3 Model B~~ Raspi 4 with 4GB Ram and offical 7" Touch Display

## Fixing a boot hang

With the current debian there seams to be an error with the touch driver. 
Added the following to `/boot/config`:
`dtparam=i2c_vc_baudrate=50000`

## Fixing a WIFI-Problem

it seems that the raspi 4 has issues with video signals interfering with wifi... (or the other way around :) ). Anyways: Switching wifi to 5 Ghz instead of 2.4 GHz solves the issues.

```bash
#add to /etc/rc.local before exit 0 line
/sbin/iwconfig wlan0 power off
```



### Tweaks

## Screensaver (blank screen / off)

Screensaver interval can be set via `xset s 90` (90 seconds)
Maybe you have to `export DISPLAY=:0`first.

## Unclutter // no cursor

unclutter caused some high load issues. So i used the following method to hide the cursor:

```
  sudo nano /etc/lightdm/lightdm.conf

    changed:
    # xserver-command = X

    to:
    xserver-command = X -nocursor
```

## Dim Backlight

as root: `echo 60 > /sys/class/backlight/10-0045/brightness`
it takes values between 0 and 255;

## Chromium Kiosk Mode

Good instructions on setting up a Raspberry for Kiosk Mode Chromium
[https://werner.rothschopf.net/201501_raspberry_kioskmode.htm]

`chromium-browser --noerrdialogs --disable-translate --kiosk --incognito http://whereyouwannago.com`

## Purpose of the start-scripts

put the start scripts on your desktop and adjust the path to the motion sensor python script. This way you can easily start the 
motionsensor and the chromium kiosk mode manually from you touch screen on reboot.
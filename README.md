# "Rumble Feedback" for Live for Speed
Tiny program that allows me to have rumble on several points of my SimRacing "Rig" (chair, table, steering wheel, pedals)
The goal is to feel each wheel as they drift or bump into something, and be able to correct when necessary. 

Ah! I forgot the why: my old Steering Wheel does not have Force Feedback and I don't like throwing things away.

## TL;DR
Keep reading, there is no real TL;DR here

## Requirements
An Arduino and some vibration motors
Live For Speed with OutGauge  and OutSime enabled
Arduino IDE
Access to run Sudo

, all of which you'll need to search the interwebs for further information


## How to run:
- Connect a vibration motor to your arduino (Pin 2). Glue it somewhere on your Rig (I used the steering wheel's dashboard)
- Install arduino_code/actuators/actuators.ino to your Arduino (keep it connected through a USB cable)
- $ make
- start LFS

, again, if you have any doubts, use Google or ChatGPT or whatevs to do your own research like I did.

## WARRANTY
NO WARRANTY PROVIDED.
USE THIS AT YOUR OWN RISK!
I AM NOT RESPONSIBLE FOR ANYTHING THAT MAY HAPPEN TO YOU OR SOMEONE, OR SOMETHING AROUND YOU IF YOU FOLLOW MY STEPS!

## Issues
- I need to have Arduino IDE's Serial Monitor enabled (And run the program with Sudo) for this to work
- 

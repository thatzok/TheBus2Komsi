# TheBus2Komsi

TheBus2Komsi is an API-Client for the "TheBus" bus simulator.<br>

TheBus2Komsi reads information (for speed, lamps, etc.) from the TheBus telemetry-API and sends them to the serial port (USB) using the KOMSI protocol.

An Arduino/ESP32 or similar connected to the USB port can then read these messages and display the data on a bus dashboard (e.g. speed on a speedometer, lamp lighting, etc.).

## Usage

The configuration is done via the file TheBus2Komsi.ini, which must be located in the same directory as TheBus2Komsi.exe.

```
# TheBus2Komsi.ini
# This file must be in the same directory as TheBus2Komsi.exe
#
# Normally you only need to change the portname to the one your are using
# 
# If you don't know which comport your Arduino/ESP32 is connected to, you can start the program with
# TheBus2Komsi -l

[default]
portname = com22
baudrate = 115200
sleeptime = 200
ip = 127.0.0.1
```


To get a list of all command line parameters, start the program with the "--help" option.

  ```sh
  TheBus2Komsi --help
  ```


## Testing if the API works

To test whether the connection to the API (called "Telemetry" in Game) of TheBus works without having set up a serial port, you can start the program "TheBusTestAPI" instead of "TheBus2Komsi".

You should see similar output:
  ```
Verbose Mode enabled.
TheBus2Komsi has started. Have fun!
Bitte einsteigen und hinsetzen.
  ```

And as soon as you sit in the driver's seat of a bus and, for example, turn on the ignition, you should see the variables read and changing:

  ```
Verbose Mode enabled.
TheBus2Komsi has started. Have fun!
Bitte einsteigen und hinsetzen.
Hingesetzt. Jetzt gehts los!
doors: 0 -> 1
fixing_brake: 0 -> 1
ignition: 0 -> 1
lights_stop_brake: 0 -> 1
lights_front_door: 0 -> 1
fuel:  0 -> 1
fuel:  1 -> 4
fuel:  4 -> 7
fuel:  7 -> 10
fuel:  10 -> 13
fuel:  13 -> 16
fuel:  16 -> 19
fuel:  19 -> 22
fuel:  22 -> 25
fuel:  25 -> 28
fuel:  28 -> 31
fuel:  31 -> 34
fuel:  34 -> 37
fuel:  37 -> 40
fuel:  40 -> 43
fuel:  43 -> 46
fuel:  46 -> 49
fuel:  49 -> 52
fuel:  52 -> 55
fuel:  55 -> 58
fuel:  58 -> 61
fuel:  61 -> 64
fuel:  64 -> 67
fuel:  67 -> 70
fuel:  70 -> 73
fuel:  73 -> 76
fuel:  76 -> 79
fuel:  79 -> 82
fuel:  82 -> 85
fuel:  85 -> 88
fuel:  88 -> 91
fuel:  91 -> 94
fuel:  94 -> 97
fuel:  97 -> 100
  ```

If you see such or similar output, then reading the API is working.

If you don't see the variables, you can start the program with the "-d" (debug) option, which gives you a lot more information, although this might be too cryptic for some.

Have fun!

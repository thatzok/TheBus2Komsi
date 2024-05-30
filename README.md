# TheBus2Komsi

TheBus2Komsi is an API-Client for the TheBus Bus Simulator.<br>

TheBus2Komsi reads information (for speed, lamps, etc.) from the TheBus telmetry-API and sends them to the serial port (USB) using the KOMSI protocol.

An Arduino/ESP32 or similar connected to the USB port can then read these messages and display the data on a bus dashboard (e.g. speed on a speedometer, lamp lighting, etc.).



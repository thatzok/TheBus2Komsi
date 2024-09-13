# Mapping of TheBus API-Values to KOMSI-Values

API-Call is (on local machine): http://127.0.0.1:37337/Vehicles/Current


| API-Value(s) read        | KOMSI-Code | KOMSI-Name          | KOMSI-Values                      | Mapping                                 |          
|--------------------------|------------|---------------------|-----------------------------------|-----------------------------------------|
| `ActorName`                |            |                     |                                   | read from the API but not used in KOMSI |      
| `IgnitionEnabled`          | A          | Ignition            | 0=off, 1=on                       | false -> 0, true -> 1                   |    
| `EngineStarted`            | B          | Engine              | 0=off, 1=on                       | false -> 0, true -> 1                   |       
| `PassengerDoorsOpen`       | C          | PassengerDoorsOpen  | 0=all closed, 1=at least one open | false -> 0, true -> 1                   |    
| `IndicatorState`           | D          | Indicator           | 0=off, 1=left on, 2=right on      | 0 -> 0,-1 -> 1, 1 -> 2                  |
| `FixingBrake`              | E          | FixingBrake         | 0=off, 1=on                       | false -> 0, true -> 1                   | 
| `WarningLights`            | F          | WarningLights       | 0=off, 1=on                       | false -> 0, true -> 1                   |   
| `AllLamps.LightHeadlight`  | G          | MainLights          | 0=off, 1=on                       | int of value                            |  
| `ButtonLight Door 1`       | H          | FrontDoor           | 0=closed, 1=open                  | int of value                            |   
| `ButtonLight Door 2`       | I          | SecondDoor          | 0=closed, 1=open                  | int of value                            | 
| `LED StopRequest`          | K          | StopRequest         | 0=off, 1=on                       | int of value                            |
| `ButtonLight BusStopBrake`  | L          | StopBrake           | 0=off, 1=on                       | int of value                            |
| `AllLamps.LightTraveling`   | M          | HighBeam            | 0=off, 1=on                       | int of value                            |
|                             | O          | SimulatorType       | 0=OMSI 2, 1=TheBus                | fixed value 1 at start                 |
| `AllowedSpeed`             | s          | MaxSpeed            | integer                           | abs of round of value                   |                    
| `DisplayFuel`              | x          | Fuel                | integer (0 ... 100)               | round of (100*value)                    
| `Speed`                    | y          | Speed               | integer                           | abs of round of value                   |



Remarks:
* `AllLamps.LightTraveling` means the variable `LightTraveling` in the Json array `AllLamps`.
* `ButtonLight BusStopBrake` is really a variable name. Many API variable names contain spaces.

History:
* `AllLamps.LightMain` does not change it's value anymore (bug or intended?), we use  `AllLamps.LightHeadlight` instead as a workaround.

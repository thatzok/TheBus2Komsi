# TheBus2Komsi
[![Build](https://github.com/thatzok/TheBus2Komsi/actions/workflows/build.yml/badge.svg)](https://github.com/thatzok/TheBus2Komsi/actions/workflows/build.yml)

[English](README.md) | **Deutsch**

TheBus2Komsi ist ein API-Client für den Bussimulator "TheBus".<br>

TheBus2Komsi liest Informationen (für Geschwindigkeit, Lampen, etc.) aus der TheBus Telemetrie-API und sendet diese über den seriellen Port (USB) unter Verwendung des [KOMSI-Protokolls](https://github.com/thatzok/Komsi-Protocol).

Ein an den USB-Anschluss angeschlossener Arduino/ESP32 oder ähnliches kann diese Nachrichten dann lesen und die Daten auf einem Bus-Dashboard anzeigen (z. B. Geschwindigkeit auf einem Tacho, Lampenbeleuchtung, etc.).

## Verwendung

Die Konfiguration erfolgt über die Datei TheBus2Komsi.ini, die sich im selben Verzeichnis wie die TheBus2Komsi.exe befinden muss.

```
# TheBus2Komsi.ini
# Diese Datei muss sich im selben Verzeichnis wie TheBus2Komsi.exe befinden
#
# Normalerweise müssen Sie nur den Portnamen auf den von Ihnen verwendeten ändern
# 
# Wenn Sie nicht wissen, an welchen Comport Ihr Arduino/ESP32 angeschlossen ist, können Sie das Programm mit
# TheBus2Komsi -l starten
[default]
portname = com8
baudrate = 115200
sleeptime = 200
ip = 127.0.0.1
```

Um eine Liste aller Kommandozeilenparameter zu erhalten, starten Sie das Programm mit der Option "--help".

  ```sh
  TheBus2Komsi --help
  ```

## Testen, ob die API funktioniert

Um zu testen, ob die Verbindung zur API (im Spiel "Telemetry" genannt) von TheBus funktioniert, ohne einen seriellen Port eingerichtet zu haben, können Sie statt "TheBus2Komsi" das Programm "TheBusTestAPI" starten.

Sie sollten eine ähnliche Ausgabe sehen:
  ```
Verbose Mode enabled.
TheBus2Komsi has started. Have fun!
Bitte einsteigen und hinsetzen.
  ```

Und sobald Sie auf dem Fahrersitz eines Busses sitzen und beispielsweise die Zündung einschalten, sollten Sie sehen, wie die Variablen gelesen werden und sich ändern:

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

Wenn Sie eine solche oder ähnliche Ausgabe sehen, dann funktioniert das Lesen der API.

Wenn Sie die Variablen nicht sehen, können Sie das Programm mit der Option "-d" (Debug) starten, die Ihnen viel mehr Informationen liefert, auch wenn dies für manche zu kryptisch sein mag.

Viel Spaß!

## Lizenz

Dieses Programm ist freie Software: Sie können es unter den Bedingungen der GNU General Public License, wie von der Free Software Foundation veröffentlicht, weitergeben und/oder modifizieren, entweder in Version 3 der Lizenz oder (nach Ihrer Wahl) in jeder späteren Version.

Dieses Programm wird in der Hoffnung verteilt, dass es nützlich sein wird, aber OHNE JEDE GEWÄHRLEISTUNG; sogar ohne die implizite Gewährleistung der MARKTGÄNGIGKEIT oder EIGNUNG FÜR EINEN BESTIMMTEN ZWECK. Siehe die [GNU General Public License](LICENSE) für weitere Details.

Sie sollten eine Kopie der GNU General Public License zusammen mit diesem Programm erhalten haben. Wenn nicht, siehe <https://www.gnu.org/licenses/>.

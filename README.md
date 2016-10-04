# voebb-calendar-sync

`voebb-calendar-sync` ist ein kleines Programm, um die Abgabetermine aus dem Online-System des [Verbundes Öffentlicher Bibliotheken Berlins](https://www.voebb.de) in deinem [Google-Kalender](https://calendar.google.com) zu speichern.

## Installation

Um `voebb-calendar-sync` zu installieren kannst du entweder diesen Repo clonen oder dir die nur die vorkompilierte Version für dein OS ([Linux](https://github.com/pajowu/voebb-calendar-sync/blob/master/voebb_scraper-linux?raw=true), [Mac](https://github.com/pajowu/voebb-calendar-sync/blob/master/voebb_scraper-mac?raw=true), [Windows](https://github.com/pajowu/voebb-calendar-sync/blob/master/voebb_scraper.exe?raw=true)) sowie die [`client_secret.json`](https://github.com/pajowu/voebb-calendar-sync/raw/master/client_secret.json) herunterladen und in einem Verzeichnis speichern.


## Nutzung

```
Usage: voebb_scraper -u USERNAME -p PASSWORD

Options:
    -u user             voebb nutzername (11-stellige Ausweisnummer)
    -p pass             voebb password
    -h, --help          print this

```

Damit läd das Programm die deine ausgeliehenen Bücher und speichert diese in deinem Google Kalender. Bevor das erste Buch gespeichert werden kann, wird dich das Programm bitten, eine Website zu besuchen 
und dort einen bestimmten Code einzugeben, um dem Programm zugriff auf deinen Kalender zu geben.

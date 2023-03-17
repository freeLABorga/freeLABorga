<a name="readme-top"></a>


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/freeLABorga/freeLABorga">
    <img src="docs/logo_header.png" alt="Logo" style="max-height: 100px">
  </a>

<h2 align="center">freeLABorga</h2>

  <p align="center">
    Software zum Verwalten und Ausleihen von Laborgegenständen an Hochschulen
    <br />
    <br />
    <a href="https://youtu.be/HWXdl-DqLnA">View Demo Video</a>
    ·
    <a href="https://github.com/freeLABorga/freeLABorga/issues">Report Bug</a>
  </p>
</div>



<details>
  <summary>Inhaltsverzeichnis</summary>
  <ol>
    <li>
      <a href="#über-das-projekt">Über das Projekt</a>
      <ul>
        <li><a href="#features">Features</a></li>
        <li><a href="#geschrieben-mit">Geschrieben mit</a></li>
      </ul>
    </li>
    <li><a href="#ausführen">Ausführen</a></li>
    <li><a href="#benutzung">Benutzung</a></li>
    <li><a href="#lizenz">Lizenz</a></li>
    <li><a href="#kontakt">Kontakt</a></li>
  </ol>
</details>



## Über das Projekt

![Screen Shot][product-screenshot]

**freeLABorga** bietet eine einfache Möglichkeit, Hochschullabore zu verwalten. Dazu gehört neben der Inventarverwaltung auch die Möglichkeit, Laborgegenstände an Studierende auszuleihen.

Die Software entstand während eines Programmierprojekts im dritten Semester im Studiengang Informatik an der Technischen Hochschule Würzburg-Schweinfurt.

Vorgabe war, Rust für das Backend zu benutzen und JavaScript mit HTML / CSS ohne weitere Bibliotheken für das Frontend

<p align="right">(<a href="#readme-top">back to top</a>)</p>


### Features
* **Laborgegenstände** mit Name, Lagerort, Kategorien, ID, Seriennummer, Kaufpreis und -datum
* **Personen** (Studierende) mit Matrikelname, Vor-, Nachname und E-Mail
* Personen können Laborgegenstände **ausleihen**
* **Ausleihen-Log** pro Gegenstand mit Person, Verleihdatum, geplantes und tatsächliches Rückgabedatum
* Verwaltung von **Beschädigungen** der Gegenstände
* **Ausinventarisieren** von nicht mehr vorhandenen Gegenständen
* Umfangreiche **Such-** und **Filterfunktion**
* **Heatmap** auf der die Ausgaben nach Monaten gruppiert dargestellt werden 
* Frei anpassbare **Impressums-** und **Datenschutzseite**
* Vollständig **responsiv** und unterstützt sowohl **Light-** als auch **Dark-Mode**

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Geschrieben mit

* [![Rust][rust]][rust-url]
* [![HTML][html]][html-url]
* [![CSS3][css]][css-url]
* [![JavaScript][javascript]][javascript-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



## Ausführen

1. Sicherstellen, dass Rust incl. Cargo installiert ist.

2. Repository clonen oder [ZIP-Datei herunterladen](https://github.com/freeLABorga/freeLABorga/archive/refs/heads/main.zip)
   ```sh
   git clone https://github.com/freeLABorga/freeLABorga.git
   ```
  
3. In den Repository-Ordner wechseln und folgenden Befehl ausführen:
   ```sh
   cargo run
   ```

4. Einen Webbrowser öffnen und folgende URL eingeben
   ```
   http://localhost:8080/
   ```
   Wenn von einem anderen Rechner im lokalen Netzwerk zugegriffen werden soll, Firewalleinstellungen beachten und statt `localhost` die IP-Adresse des Rechners verwenden.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Benutzung

Siehe [Demo-Video](https://www.youtube.com/watch?v=HWXdl-DqLnA) (Link zu YouTube)

Um die Demo Daten zu entfernen: die Datei "db.db3" löschen. Diese wird dann automatisch ohne Daten beim nächsten Programmstart erstellt.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Lizenz

Veröffentlicht unter der Lizenz **GPL v3**. Weitere Informationen in der Datei [`LICENSE`](LICENSE).

```
 Copyright (C) 2022-2023  Nico Hoffmann, Jan Ludwig, Philipp Pfeiffer 
 
 freeLABorga is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License Version 3
 as published by the Free Software Foundation on June 29, 2007.
 
 freeLABorga is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with freeLABorga.  If not, see <http://www.gnu.org/licenses/>.
 ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



## Kontakt

* Nico Hoffmann - nicohoffmann.THWS@gmx.de
* Jan Ludwig - janludwigthws@gmail.com
* Philipp Pfeiffer - FLOSS@philipp-pfeiffer.net

Project Link: [https://github.com/freeLABorga/freeLABorga](https://github.com/freeLABorga/freeLABorga)

<p align="right">(<a href="#readme-top">back to top</a>)</p>




<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[rust]: https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://rust-lang.org

[html]: https://img.shields.io/badge/html5-%23E34F26.svg?style=for-the-badge&logo=html5&logoColor=white
[html-url]: https://html.spec.whatwg.org/multipage/

[css]: https://img.shields.io/badge/css3-%231572B6.svg?style=for-the-badge&logo=css3&logoColor=white
[css-url]: https://www.w3.org/Style/CSS/

[javascript]: https://img.shields.io/badge/javascript-%23323330.svg?style=for-the-badge&logo=javascript&logoColor=%23F7DF1E
[javascript-url]: http://www.ecmascript.org/

[product-screenshot]: docs/screenshot.jpg

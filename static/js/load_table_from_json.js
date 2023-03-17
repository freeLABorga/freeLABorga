/**
 * This file is part of freeLABorga.
 * Copyright (C) 2022-2023  Nico Hoffmann, Jan Ludwig, Philipp Pfeiffer 
 *
 * freeLABorga is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License Version 3
 * as published by the Free Software Foundation on June 29, 2007.
 *
 * freeLABorga is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with freeLABorga.  If not, see <http://www.gnu.org/licenses/>.
 */

'use strict';

const errorTitle = "FEHLER";
const errorMessageConnection = "<p>Fehler beim Abrufen der Daten.<br>Bitte überprüfen Sie die Verbindung zum Server!</p>";
const errorMessageProcessingData = "<p>Fehler beim Verarbeiten der Daten.</p>";

export class LoadTableFromJson {
    /**
     * Lädt JSON-Dokument von übergebener URL und fügt die Daten in den Tabellen-Body auf der Seite ein
     * @param {String} jsonUrl URL, von der die Tabellendaten abgerufen werden
     * @param {Object} fields  Konfigurations-Objekt; beschreibt wie die Daten eingefügt werden sollen.
     */
    constructor(jsonUrl, fields) {
        this.jsonUrl = jsonUrl;
        this.fields = fields;
        this.tableBody = document.getElementsByTagName("tbody")[0];
        this.paginationElement = document.getElementsByClassName("pagination")[0];
        this.jsonDocument = null;

        this.init();

        // Event-Listener bei Änderung der Page-Angabe (Wert nach "#" in URL)
        const classThis = this;
        window.addEventListener("popstate", function () { classThis.init() });
    }

    init() {
        this.loadJson()
            .then(() => this.fillTable())
            .then(() => this.addPagination())
            .catch(e => new ConfirmationPopup(null, errorTitle, errorMessageProcessingData + e, true));
    }

    /** 
     * JSON-Dokument von Server laden (unter Berücksichtigung der Seitennummer aus URL)
     */
    async loadJson() {        
        // Seitennummer aus URL nach "#" auslesen (mit Regex)
        const pagePattern = "#page-([0-9]+)";
        const pageNumberHash = location.hash
        const match = pageNumberHash.match(pagePattern);
        
        // Wenn keine Seitennummer in URL
        let pageNumber = "1";
        if (match != null)
            pageNumber = match[1];
        
        // GET-Parameter aus Seiten-URL lesen und der JSON-Anfrage mitgeben
        const getParams = location.href.split("?");
        let tempJsonUrl = this.jsonUrl;
        // Entscheiden ob, Parameter an URL mit "?" oder "&" angehängt werden müssen
        if (getParams.length > 1) {
            tempJsonUrl += "?" + getParams.slice(-1)[0].split("#")[0];
            tempJsonUrl += "&page=" + pageNumber;
        } else {
            tempJsonUrl += "?page=" + pageNumber;
        }

        try {
            // Abrufen des JSON-Dokuments
            const response = await fetch(tempJsonUrl);
            this.jsonDocument = await response.json();
            return true;
        } catch (e) {
            new ConfirmationPopup(null, errorTitle, errorMessageConnection + e, true);
            return false;
        }
    }


    /**
     * Tabellen-Body leeren und mit neuen Daten aus JSON-Dokument füllen
     */
    fillTable() {
        this.tableBody.innerHTML = "";
        this.jsonDocument.data.forEach(entry => {
            const newRow = document.createElement("tr");
            newRow.dataset.id = entry[this.fields.idField];
            if (this.fields.hasOwnProperty("eventListener")) {
                newRow.addEventListener("click", this.fields.eventListener);
                newRow.style.cursor = "pointer";
            }

            this.fields.cells.forEach(cellConf => {
                const newCell = this.createCell(cellConf, entry);
                newRow.appendChild(newCell);

                //Bei ausinventarisierten Gegenständen den Button zum ausleihen/zurückgeben verstecken
                if(entry["inventoried"] == false && cellConf.contentTrue == '<button id="button-borrow" class="add-button">ausleihen</button>') {
                    newCell.firstChild.style.visibility = 'hidden';
                }
            })

            //Hinzufügen von Classtag für Darstellung von ausinventarisierten Gegenständen
            if(entry["inventoried"] == false) {
                newRow.classList.add("notInventoried");
            }

            this.tableBody.appendChild(newRow);
        })
    }

    /**
     * Erstellt eine neue Zelle
     * @param cellConf Objekt für Zellenkonfiguration
     * @param entry  Daten für die Zelle aus JSON gelesen
     * @returns 
     */
    createCell(cellConf, entry) {
        const newCell = document.createElement("td");

        if (cellConf.hasOwnProperty("jsonName")) {
            if (cellConf.hasOwnProperty("valueClass")) {
                entry[cellConf.jsonName].forEach(value => {
                    const item = document.createElement("a");
                    item.href = (cellConf.hasOwnProperty("link")) ? cellConf.link + value : "#";
                    item.className = cellConf.valueClass;
                    item.innerText = value;
                    newCell.appendChild(item);
                });
            } else if (cellConf.hasOwnProperty("contentTrue") && cellConf.hasOwnProperty("contentFalse")) {
                if (entry[cellConf.jsonName] == true)
                    newCell.innerHTML = cellConf.contentTrue;
                else if (entry[cellConf.jsonName] == false)
                    newCell.innerHTML = cellConf.contentFalse;
                else
                    newCell.innerText = entry[cellConf.jsonName];

            } else if (cellConf.hasOwnProperty("linkWithFieldValue")) {
                const link = document.createElement("a");
                link.href = cellConf.linkWithFieldValue + entry[cellConf.jsonName];
                link.innerHTML = cellConf.staticContent;
                newCell.appendChild(link);

            } else {
                newCell.innerText = entry[cellConf.jsonName];
            }

        } else if (cellConf.hasOwnProperty("seperator")) {
            let counter = 0;
            let content = [];
            while (true) {
                if (!cellConf.hasOwnProperty("jsonName" + counter)) {
                    break;
                }
                content.push(entry[cellConf["jsonName" + counter]])
                counter++;
            }
            newCell.innerHTML = content.join(cellConf.seperator);

        } else if (cellConf.hasOwnProperty("staticContent")) {
            newCell.innerHTML = cellConf.staticContent;
        }

        if (cellConf.hasOwnProperty("cellClass"))
            newCell.className = cellConf.cellClass;

        if (cellConf.hasOwnProperty("eventListener")) {
            newCell.firstChild.addEventListener("click", cellConf.eventListener);
        }
        return newCell;
    }

    addPagination() {
        const pageNumber = this.jsonDocument.pageNumber;
        const pagesTotal = this.jsonDocument.pagesTotal;
        const itemsOnPage = this.jsonDocument.itemsOnPage;
        const itemsTotal = this.jsonDocument.itemsTotal;

        if (pagesTotal > 1) {

            this.paginationElement.innerHTML = "";

            const paginationTextElement = document.createElement("span");
            paginationTextElement.classList.add("pagination-text");
            paginationTextElement.innerText = `${itemsOnPage} von ${itemsTotal} Elementen`;
            this.paginationElement.appendChild(paginationTextElement);

            const paginationSelectElement = document.createElement("ul");

            // Pfeil für vorherige Seite (nur wenn Seite verfügbar)
            if (pageNumber > 1)
                paginationSelectElement.innerHTML += `<li><a href="#page-${pageNumber - 1}"><i class="fa-solid fa-angle-left"></i></a></li>`

            // Maximal zwei Seiten vor und zwei Seiten nach aktueller Seite in Pagination anzeigen
            let pageDisplayStart = pageNumber - 2;
            let pageDisplayEnd = pageNumber + 2;

            while(pageDisplayStart < 1) {
                pageDisplayStart++;
                pageDisplayEnd++;
            }

            while(pageDisplayEnd > pagesTotal) {
                pageDisplayStart--;
                pageDisplayEnd--;
            }

            if(pageDisplayStart < 1) {
                pageDisplayStart = 1;
            }

            // Pagination erstellen
            for (let page = pageDisplayStart; page <= pageDisplayEnd; page++) {
                const pageNumberItem = document.createElement("li");
                
                if (page == pageNumber) // aktuelle Seite markieren
                    pageNumberItem.classList.add("active");

                const pageNumberLink = document.createElement("a");
                pageNumberLink.href = "#page-" + page;
                pageNumberLink.innerText = page;
                pageNumberLink.addEventListener("click", window.scrollTo(0,0));

                pageNumberItem.appendChild(pageNumberLink);
                paginationSelectElement.appendChild(pageNumberItem);
            }

            // Pfeil für nächste Seite (nur wenn Seite verfügbar)
            if (pageNumber < pagesTotal)
                paginationSelectElement.innerHTML += `<li><a href="#page-${pageNumber + 1}"><i class="fa-solid fa-angle-right"></i></a></li>`;

            this.paginationElement.appendChild(paginationSelectElement);
        }

    }




}

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

function CreateTableDataHeatmap(year, jan, feb, mar, apr, may, jun, jul, aug, sep, oct, nov, dec) {
    this.year = year;
    this.jan = jan;
    this.feb = feb;
    this.mar = mar;
    this.apr = apr;
    this.may = may;
    this.jun = jun;
    this.jul = jul;
    this.aug = aug;
    this.sep = sep;
    this.oct = oct;
    this.nov = nov;
    this.dec = dec;
}

function addToTableHeatmap(content) {
    const tbl = document.getElementById("table-heatmap");
    const tblBody = document.getElementsByTagName("tbody")[0];
    for(let i = 0; i < content.length; i++) {
        let months = [
            content[i].jan,
            content[i].feb,
            content[i].mar,
            content[i].apr,
            content[i].may,
            content[i].jun,
            content[i].jul,
            content[i].aug,
            content[i].sep,
            content[i].oct,
            content[i].nov,
            content[i].dec
        ];
        
        let row = tblBody.insertRow();
        
        let year = row.insertCell(0);
        year.innerText = content[i].year;
        year.classList.add("year");

        for(let j = 0; j < months.length; j++) {
            let x = row.insertCell(j+1);
            x.innerText = months[j];
            let classTagColor = calcColor(months[j]);
            x.classList.add(classTagColor);
        }
    }
}

//Bestimmt die Farbe der einzelnen Zellen. Hier wird die Anzahl der verschiedenen Stufen festgelegt
function calcColor(x) {
    switch (true) {
        case x == 0:
            return "row0";
        case x < 100:
            return "row100";
        case x < 500:
             return "row500";
        case x < 1000:
            return "row1000";
        case x < 5000:
            return "row5000";
        case x < 10000:
            return "row10000";
        default:
            return "row10000i";
    }
}

//Erstellen und Befüllen der Heatmap mit den angeforderten Daten
async function fillHeatmapTable() {
    var jsonDocument = null;
    var response = null;

    const jsonUrl = "api/heatmap";

    try {
        response = await fetch(jsonUrl);
        jsonDocument = await response.json();
        
    } catch (e) {
        new ConfirmationPopup(null, "FEHLER", "<p>Fehler beim Abrufen der Daten.<br>Bitte überprüfen Sie die Verbindung zum Server!</p>" + e, true);
        return false;
    }

    var minYear = Infinity;

    jsonDocument.data.forEach(entry => {
        let currentDate = new Date(entry["buydate"]);
        let year = currentDate.getFullYear();
        if(minYear > year) {
            minYear = year;
        }
    });

    let currentYear = new Date().getFullYear();
    let numberOfYears = currentYear - minYear + 1;

    let years = new Array(numberOfYears);

    for(let i = 0; i < numberOfYears; i++) {
        years[i] = new Array(12);
    }

    for(let i = 0; i < years.length; i++) {
        for(let j = 0; j < years[i].length; j++) {
            years[i][j] = 0.0;
        }
    }

    jsonDocument.data.forEach(entry => {
        let currentDate = new Date(entry["buydate"]);
        let year = currentDate.getFullYear();
        let month = currentDate.getMonth();
        
        let priceString = entry["price"];
        let priceEuroString = priceString.slice(0, -5);
        let priceCentString = priceString.slice(-4, -2);
        let number = parseFloat(priceEuroString + '.' + priceCentString);
        
        //Vermeiden von falschen Berechnungen, da parseFloat nicht 100% genau
        years[year - minYear][month] += number;
        years[year - minYear][month] = Math.round(years[year - minYear][month] * 100) / 100;
    });

    for(let i = 0; i < years.length; i++) {
        let row = new CreateTableDataHeatmap(minYear + i, years[i][0], years[i][1], years[i][2], years[i][3], years[i][4], years[i][5], years[i][6], years[i][7], years[i][8], years[i][9], years[i][10], years[i][11]);
        let rows = new Array();
        rows.push(row);
        addToTableHeatmap(rows);
    }
}

fillHeatmapTable();

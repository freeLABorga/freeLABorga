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

import {LoadTableFromJson} from "./load_table_from_json.js";
import { AddEditDeletePopup as AddEditDeletePopup } from "./add_edit_popup.js";
import { BorrowTakebackPopup as BorrowTakebackPopup } from "./borrow_takeback_popup.js";

const gegenstaendeApiUrl= "api/gegenstand";
const borrowApiUrl = "api/lend"
const takebackApiUrl = "api/lend"


function showAddPersonPopup(e) {
    addEditDelete.showAddPopup();
}

document.getElementById("button-add-thing").addEventListener("click", showAddPersonPopup);

function changeBorrowStatus(row, state){
    if(state){
        if(row.querySelector("td:nth-child(4)").innerHTML == '<i class="fa-solid fa-xmark"></i>'){
            row.querySelector("td:nth-child(4)").innerHTML = '<i class="fa-solid fa-check"></i>';
        }
        else {
            row.querySelector("td:nth-child(4)").innerHTML = '<i class="fa-solid fa-xmark"></i>';
        }
        console.log("jaaaa");
    }
}
export function showBorrowOrTakeBackPopup(){
    // Find the nearest ancestor tr element that contains this button
    let row = this.closest("tr");
    let htmlButton = row.querySelector("td:nth-child(6)").innerHTML;
    if(htmlButton == ('<button id="button-borrow" class="add-button">ausleihen</button>')){
        showBorrowPopup(row);
    }
    else if(htmlButton == ('<button id="button-takeback" class="add-button">zurückgeben</button>')){
        showTakeBackPopup(row);
    }
}

function showBorrowPopup(row){  
    // Find the nearest ancestor tr element that contains this button and get the name
    let thingName = row.querySelector("td:nth-child(1)").textContent;
    let thingId = row.querySelector("td:nth-child(3)").textContent;

    // This is the code that will run when the button is clicked
    //const popup = new ConfirmationPopup((state) => changeBorrowStatus(row, state), "Artikel ausleihen?", "<p><label>Person: <input list='personen' id='person-input'></label></p>" + "<p><label>MatNr.: <input type='text' id='matNR-input'></label></p>" + "<p><label>Leihdatum: <input type='date' id='borrow-date-input'></label></p>" + "<p><label>Zeitraum: <input list='zeitraueme' id='borrow-length-input'></label></p>"+ "<p><label>ReturnDate: <input type='date' id='borrow-back-date-input'></label></p>"+ `<p>Möchten Sie den Artikel "<b>${thingName}</b>" wirklich ausleihen?</p>`, false);
    borrow.showBorrowPopup(thingId);
}

function showTakeBackPopup(row){
    // Find the nearest ancestor tr element that contains this button and get the name
    let thingName = row.querySelector("td:nth-child(1)").textContent;
    let thingId = row.querySelector("td:nth-child(3)").textContent;
    // This is the code that will run when the button is clicked
    //const popup = new ConfirmationPopup((state) => changeBorrowStatus(row, state), "Artikel zurückgeben?", "<p><label>Person: <input list='personen' id='person-input'></label></p>" + "<p><label>MatNr.: <input type='text' id='matNR-input'></label></p>" + "<p><label>ReturnDate: <input type='date' id='borrow-back-date-input'></label></p>"+ `<p>Möchten Sie den Artikel "<b>${thingName}</b>" wirklich zurückgeben?</p>`, false);
    takeback.showTakeBackPopup(thingId);
}


const itemsFields = {
    idField: "id",
    cells: [
        {
            jsonName: "name"
        },
        {
            jsonName: "categories",
            valueClass: "cat-item",
            link: "gegenstaende.html?cat="
        },
        {
            jsonName: "id"
        },
        {
            jsonName: "available",
            contentTrue: '<i class="fa-solid fa-check"></i>',
            contentFalse: '<i class="fa-solid fa-xmark"></i>'
        },
        {
            jsonName: "id",
            cellClass: "detail-view",
            staticContent: '<i class="fa-solid fa-magnifying-glass-plus"></i>',
            linkWithFieldValue: "detailansicht.html?id="
        },
        {
            jsonName: "available",
            contentTrue: '<button id="button-borrow" class="add-button">ausleihen</button>',
            contentFalse: '<button id="button-takeback" class="add-button">zurückgeben</button>',
            eventListener: showBorrowOrTakeBackPopup
        }

    ]
}


const t = new LoadTableFromJson(gegenstaendeApiUrl, itemsFields, false);


const itemsInputFields = {
    popupTitleAdd: "Gegenstand hinzufügen",
    popupTitleEdit: "Gegenstand bearbeiten",
    popupTitleDelete: "Gegenstand löschen",
    popupMessageDelete: "<p>Möchten Sie den Gegenstand <strong>{0}</strong> wirklich löschen?</p>",
    getDataUrl: gegenstaendeApiUrl,
    optionListUrl: "api/kategorie_lagerplatz",
    eventAfterAction: () => t.init(),
    fields: [
        {
            element: "input",
            displayName: "Name",
            jsonName: "name",
            type: "text",
            required: true,
            tableColumn: 0
        }, 
        {
            element: "input",
            displayName: "Lagerort",
            jsonName: "place",
            type: "text",
            required: true
        },
        {
            element: "input",
            displayName: "Kategorien",
            jsonName: "categories",
            type: "text",
            multiple: true,
            tableColumn: 1,
        },
        {
            element: "input",
            displayName: "ID",
            jsonName: "id",
            type: "text",
            required: true,
            tableColumn: 3
        },
        {
            element: "input",
            displayName: "Seriennummer",
            jsonName: "serialnumber",
            type: "text"
        },
        {
            element: "input",
            displayName: "Kaufpreis (in EUR)",
            jsonName: "price",
            type: "price"
        },
        {
            element: "input",
            displayName: "Kaufdatum",
            jsonName: "buydate",
            type: "date"
        },
    ]
}

const addEditDelete = new AddEditDeletePopup(gegenstaendeApiUrl, itemsInputFields);

const BorrowInputFields = {
    popupTitleBorrow: "Gegenstand ausleihen",
    popupTitleTakeback: "Gegenstand zurückgeben",
    popupTitleDelete: "Gegenstand löschen",
    popupMessageDelete: "<p>Möchten Sie den Gegenstand <strong>{0}</strong> wirklich löschen?</p>",
    getDataUrl: gegenstaendeApiUrl,
    optionLists: "/api/schnellsuche?search=",
    eventAfterAction: () => t.init(),
    fields: [
        {
            element: "input",
            displayName: "MatNr",
            jsonName: "idPerson",
            required: true,
            type: "text",
            tableColumn: 1,
            optionListKey: "persons"
        },
        {
            element: "input",
            displayName: "Leihdatum",
            jsonName: "lendDate",
            type: "date",
            required: true,
            tableColumn: 2
        },
        {
            element: "input",
            displayName: "Rückgabedatum",
            jsonName: "plannedReturnDate",
            type: "date",
            required: true,
            tableColumn: 3
        },
    ]
}

const borrow = new BorrowTakebackPopup(borrowApiUrl, BorrowInputFields);

const TakeBackInputFields = {
    popupTitleBorrow: "Gegenstand ausleihen",
    popupTitleTakeback: "Gegenstand zurückgeben",
    getDataUrl: gegenstaendeApiUrl,
    eventAfterAction: () => t.init(),
    fields: [
        {
            element: "input",
            displayName: "Rückgabedatum",
            jsonName: "actualReturnDate",
            type: "date",
            required: true,
            tableColumn: 3
        },
    ]
}
const takeback = new BorrowTakebackPopup(takebackApiUrl, TakeBackInputFields);

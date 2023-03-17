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

import { AddEditDeletePopup as AddEditDeletePopup } from "./add_edit_popup.js";
import { BorrowTakebackPopup as BorrowTakebackPopup } from "./borrow_takeback_popup.js";

const damageApiUrl = "api/damage";
const borrowApiUrl = "api/lend";
const takebackApiUrl = "api/lend";
const gegenstaendeApiUrl= "api/gegenstand";
var globalDamage = false;
var gegenstandId;

//fordert Gegenstand mit passender ID an und stellt Daten in Haupttabell dar
async function fillMainTable() {
    var urlString = window.location.search;
    var url = new URLSearchParams(urlString);
    var objectID  = url.get("id");
    gegenstandId = objectID;
    
    var jsonDocument = null;
    var response = null;
    
    const jsonUrl = "api/gegenstand?id=" + objectID + "&page=1";

    try {
        response = await fetch(jsonUrl);
        jsonDocument = await response.json();
    } catch (e) {
        new ConfirmationPopup(null, "FEHLER", ""+e, true);
        return false;
    }

    jsonDocument.data.forEach(entry => {
        if(entry["id"] == objectID) {
            document.getElementById("Textfeld_Name").innerHTML = entry["name"];
            document.getElementById("Textfeld_Lagerort").innerHTML = entry["place"];

            var string = '';
            entry["categories"].forEach(cat => {
                string += `<a href="gegenstaende.html?cat=${cat}" class="cat-item">${cat}</a>`;
            })

            document.getElementById("Textfeld_Kategorien").innerHTML = string;

            document.getElementById("Textfeld_ID").innerHTML = entry["id"];
            document.getElementById("Textfeld_SN").innerHTML = entry["serialnumber"];

            if(entry["available"]) {
                document.getElementById("btn-borrowOrReturn").innerHTML = "Ausleihen <i class='fa-solid fa-arrow-up-from-bracket'>";
                document.getElementById("Textfeld_Verfügbarkeit").innerHTML = "<i class='fa-solid fa-check'>";
            } else {
                document.getElementById("btn-borrowOrReturn").innerHTML = "Zurückgeben <i class='fa-solid fa-rotate-left'>";
                document.getElementById("Textfeld_Verfügbarkeit").innerHTML = "<i class='fa-solid fa-xmark'>";
            }

            if(entry["inventoried"]) {
                document.getElementById("Textfeld_Inventarisiert").innerHTML = "<i class='fa-solid fa-check'>";
            } else {
                document.getElementById("Textfeld_Inventarisiert").innerHTML = "<i class='fa-solid fa-xmark'>";
            }

            document.getElementById("Textfeld_Preis").innerHTML = entry["price"].toFixed(2).replace(".", ",") + " €";
            
            document.getElementById("Textfeld_Kaufdatum").innerHTML = entry["buydate"];
        }
    })
    hideButtons();
    setInvButton();
}

fillMainTable();

//fordert alle Beschädigungen zu Gegenstand an und stellt sie in Zustandstabelle dar
async function fillZustandsTable() {
    var urlString = window.location.search;
    var url = new URLSearchParams(urlString);
    var objectID  = url.get("id");
    
    var jsonDocument = null;
    var response = null;

    var page = 1
    
    const jsonUrl = "api/damage?search=" + objectID + "&page=1";

    try {
        response = await fetch(jsonUrl);
        jsonDocument = await response.json();
    } catch (e) {
        new ConfirmationPopup(null, "FEHLER", ""+e, true);
        return false;
    }

    jsonDocument.data.forEach(entry => {
        if(objectID == entry["labItemId"]) {
            let row = new CreateTableDataZustand(entry["date"], entry["description"], entry["repaired"], entry["id"]);
            let rows = new Array();
            rows.push(row);
            addToTableZustand(rows);
        }
    })
    checkZustand();
}

function CreateTableDataZustand(date, beschaedigung, behoben, id, labItemId) {
    this.date = date;
    this.beschaedigung = beschaedigung;
    this.behoben = behoben;
    this.id = id;
    this.labItemId = labItemId;
}

function addToTableZustand(content) {
    const tbl = document.getElementById("Zustandshistorie");
    for(let i = 0; i < content.length;i++){
        let row = tbl.insertRow();

        row.setAttribute("data-id", content[i].id);
        row.dataset.id = content[i].id;

        let date = row.insertCell(0);
        date.innerText = content[i].date;

        let beschaedigung = row.insertCell(1);
        beschaedigung.innerText = content[i].beschaedigung;

        let fixed = row.insertCell(2);
        if(content[i].behoben == true) {
            fixed.innerHTML = '<i class="fa-solid fa-check"></i>';
        } else {
            fixed.innerHTML = '<i class="fa-solid fa-xmark"></i>';
            globalDamage = true;
        }
        
        let btnEdit = row.insertCell(3);
        btnEdit.innerHTML = '<button class="btn-edit-zustand"><i class="fa-solid fa-pen-to-square"></i></button>';
        
        let btnsEditZustand = document.querySelectorAll(".btn-edit-zustand");
        btnsEditZustand.forEach(btn =>
            btn.addEventListener("click", showEditDamagePopup)
        );

        let btnDelete = row.insertCell(4);
        btnDelete.innerHTML = '<button class="btn-delete-zustand"><i class="fa-solid fa-trash-can"></i></button>';
        
        let btnsDeleteZustand = document.querySelectorAll(".btn-delete-zustand");
        btnsDeleteZustand.forEach(btn =>
            btn.addEventListener("click", showDeleteDamagePopup)  
        );
    }
    checkZustand();
}

fillZustandsTable();

function checkZustand(){
    if(globalDamage) {
        document.getElementById("Textfeld_Zustand").innerHTML = "beschädigt";
    } else {
        document.getElementById("Textfeld_Zustand").innerHTML = "intakt";
    }
}

//fordert die Ausleihen und dazugehörigen Personen an und stellt sie in Ausleihtabelle dar
async function fillAusleiheTable() {
    var urlString = window.location.search;
    var url = new URLSearchParams(urlString);
    var objectID  = url.get("id");
    
    var jsonDocumentAusleihen = null;

    var response = null;
    
    const jsonUrl = "api/lend?id=" + objectID + "&page=1";

    try {
        response = await fetch(jsonUrl);
        jsonDocumentAusleihen = await response.json();
    } catch (e) {
        new ConfirmationPopup(null, "FEHLER", ""+e, true);
        return false;
    }

    jsonDocumentAusleihen.data.forEach(entry => {
        if(objectID == entry["idLabItem"]) {
                    let row = new CreateTableDataAusleihe(entry["firstname"], entry["lastname"], entry["lendDate"], entry["plannedReturnDate"], entry["actualReturnDate"]);
                    let rows = new Array();
                    rows.push(row);
                    addToTableAusleihe(rows);
        }
    })
}

fillAusleiheTable();

function CreateTableDataAusleihe(name, lastName, date, plannedDate, actualDate) {
    this.name = name;
    this.lastName = lastName;
    this.date = date;
    this.plannedDate = plannedDate;
    this.actualDate = actualDate;
}

function addToTableAusleihe(content) {
    const tbl = document.getElementById("Ausleihhistorie");
    for(let i = 0; i < content.length; i++) {
        let row = tbl.insertRow();
        
        let name = row.insertCell(0);
        name.innerText = content[i].lastName + ', ' + content[i].name;

        let date = row.insertCell(1);
        date.innerText = content[i].date;

        let plannedDate = row.insertCell(2);
        plannedDate.innerText = content[i].plannedDate;

        let actualDate = row.insertCell(3);
        actualDate.innerText = content[i].actualDate;
    }
}

//Entscheiden, ob Gegenstand inventarisiert oder ausinventarisiert werden kann
function setInvButton() {
    if(document.getElementById("Textfeld_Inventarisiert").innerHTML == '<i class="fa-solid fa-xmark"></i>') {
        document.getElementById("btn-ausinv").addEventListener("click", showInvPopup);
        document.getElementById("btn-ausinv").innerHTML = 'Inventarisieren <i class="fa-solid fa-arrow-down-to-square"></i>';
    } else {
        document.getElementById("btn-ausinv").addEventListener("click", showAusinvPopup);
        document.getElementById("btn-ausinv").innerHTML = 'Ausinventarisieren <i class="fa-solid fa-eject"></i>';
    }
}

function showAusinvPopup (e) {
    ninventoryPopup.showDeletePopup(e);
}

function showInvPopup(e) {
    inventoryPopup.showDeletePopup(e);
}

const ninventoryInputFields = {
    popupTitleAdd: "Gegenstand hinzufügen",
    popupTitleEdit: "Gegenstand inventarisieren",
    popupTitleDelete: "Gegenstand ausinventarisieren",
    popupMessageDelete: "<p>Möchten Sie den Gegenstand <strong>{0}</strong> wirklich ausinventarisieren?</p>",
    getDataUrl: "api/gegenstand",
    eventAfterAction: () => location.reload(),
    fields: [

    ]
}

const inventoryInputFields = {
    popupTitleAdd: "Gegenstand hinzufügen",
    popupTitleEdit: "Gegenstand inventarisieren",
    popupTitleDelete: "Gegenstand inventarisieren",
    popupMessageDelete: "<p>Möchten Sie den Gegenstand <strong>{0}</strong> wirklich inventarisieren?</p>",
    getDataUrl: "api/gegenstand",
    eventAfterAction: () => location.reload(),
    fields: [

    ]
}

const ninventoryPopup = new AddEditDeletePopup("api/gegenstand/ninventory", ninventoryInputFields);
const inventoryPopup = new AddEditDeletePopup("api/gegenstand/inventory", inventoryInputFields);


function showEditThingPopup(e) {
    addEditDeleteThing.showEditPopup(e);
}

function showDeleteThingPopup(e) {
    addEditDeleteThing.showDeletePopup(e);
}

const thingsInputFields = {
    popupTitleAdd: "Gegenstand hinzufügen",
    popupTitleEdit: "Gegenstand bearbeiten",
    popupTitleDelete: "Gegenstand löschen",
    popupMessageDelete: "<p>Möchten Sie den Gegenstand <strong>{0}</strong> wirklich löschen?</p>",
    getDataUrl: "api/gegenstand",
    optionListUrl: "api/kategorie_lagerplatz",
    deleteAndSwitch: true,
    eventAfterAction: () => fillMainTable(),
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

const addEditDeleteThing = new AddEditDeletePopup("api/gegenstand", thingsInputFields);


document.getElementById("btn-edit").addEventListener("click", showEditThingPopup);
document.getElementById("btn-delete").addEventListener("click", showDeleteThingPopup);


function showAddDamagePopup(e) {
    addEditDelete.showAddPopup();
}

function showEditDamagePopup(e) {
    addEditDelete.showEditPopup(e);
}

function showDeleteDamagePopup(e) {
    addEditDelete.showDeletePopup(e);
}

const damagesInputFields = {
    popupTitleAdd: "Beschädigung hinzufügen",
    popupTitleEdit: "Beschädigung bearbeiten:",
    popupTitleDelete: "Beschädigung löschen",
    popupMessageDelete: "<p>Möchten Sie die Beschädigung wirklich löschen?</p>",
    getDataUrl: damageApiUrl,
    eventAfterAction: () => location.reload(),
    fields: [
        {
            element: "input",
            displayName: "Datum",
            jsonName: "date",
            type: "date",
            required: true,
            tableColumn: 0
        }, 
        {
            element: "input",
            displayName: "Beschreibung",
            jsonName: "description",
            type: "text",
            required: true,
            tableColumn: 1
        },
        {
            element: "input",
            displayName: "Behoben",
            jsonName: "repaired",
            type: "checkbox",
            required: true,
            tableColumn: 2
        },
        {
            element: "input",
            displayName: "Item-ID",
            jsonName: "labItemId",
            type: "itemId",
            required: true
        }
    ]
}

const addEditDelete = new AddEditDeletePopup(damageApiUrl, damagesInputFields);

document.getElementById("btn-new").addEventListener("click", showAddDamagePopup);

export function showBorrowOrTakeBackPopup(){
    let htmlButton = document.getElementById("btn-borrowOrReturn").innerHTML;
    console.log(htmlButton);

    if(htmlButton.localeCompare('Zurückgeben <i class="fa-solid fa-rotate-left"></i>')){
        borrow.showBorrowPopup(gegenstandId);
    }else{
        takeback.showTakeBackPopup(gegenstandId);
    }
}

document.getElementById("btn-borrowOrReturn").addEventListener("click", showBorrowOrTakeBackPopup);

const BorrowInputFields = {
    popupTitleBorrow: "Gegenstand ausleihen",
    popupTitleTakeback: "Gegenstand zurückgeben",
    popupTitleDelete: "Gegenstand löschen",
    popupMessageDelete: "<p>Möchten Sie den Gegenstand <strong>{0}</strong> wirklich löschen?</p>",
    getDataUrl: gegenstaendeApiUrl,
    optionLists: "/api/schnellsuche?search=",
    eventAfterAction: () => location.reload(),
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
    eventAfterAction: () => location.reload(),
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

//versteckt Ausleih-/Zurückgeben-Button, wenn der Gegenstand nicht inventarisiert ist
function hideButtons(){
    if(document.getElementById("Textfeld_Inventarisiert").innerHTML == '<i class="fa-solid fa-xmark"></i>') {
        document.getElementById("btn-borrowOrReturn").style.visibility = 'hidden';
    }
}

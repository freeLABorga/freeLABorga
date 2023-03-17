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

import { LoadTableFromJson } from "./load_table_from_json.js";
import { AddEditDeletePopup as AddEditDeletePopup } from "./add_edit_popup.js";

const personsApiUrl = "api/person";


function showAddPersonPopup(e) {
    addEditDelete.showAddPopup();
}

document.getElementById("button-add-place").addEventListener("click", showAddPersonPopup);


function showDeletePersonPopup(e) {
    addEditDelete.showDeletePopup(e);
}

function showEditPersonPopup(e) {
    addEditDelete.showEditPopup(e);
}

const personsFields = {
    idField: "matrNr",
    cells: [
        {
            jsonName: "lastname"
        },
        {
            jsonName: "firstname"
        },
        {
            jsonName: "matrNr"
        },
        {
            jsonName: "email"
        },
        {
            cellClass: "edit-place-button",
            staticContent: "<i class='fa-solid fa-pen-to-square'></i>",
            eventListener: showEditPersonPopup
        },
        {
            cellClass: "delete-place-button",
            staticContent: "<i class='fa-solid fa-trash-can'></i>",
            eventListener: showDeletePersonPopup
        }
        
    ]
}

const t = new LoadTableFromJson(personsApiUrl, personsFields, false);

const personsInputFields = {
    popupTitleAdd: "Person hinzufügen",
    popupTitleEdit: "Person bearbeiten",
    popupTitleDelete: "Person löschen",
    popupMessageDelete: "<p>Möchten Sie die Person <strong>{0} - {1}, {2}</strong> wirklich löschen?</p><p>Die Matrikel-Nr. bleibt bei Ausleihen weiterhin gespeichert.</p>",
    getDataUrl: personsApiUrl,
    eventAfterAction: () => t.init(),
    fields: [
        {
            element: "input",
            displayName: "Nachname",
            jsonName: "lastname",
            type: "text",
            required: true,
            tableColumn: 0
        }, 
        {
            element: "input",
            displayName: "Vorname",
            jsonName: "firstname",
            type: "text",
            required: true,
            tableColumn: 1
        },
        {
            element: "input",
            displayName: "Matrikel-Nr",
            jsonName: "matrNr",
            type: "text",
            required: true,
            tableColumn: 2
        },
        {
            
            element: "input",
            displayName: "E-Mail",
            jsonName: "email",
            type: "email",
            required: true,
            tableColumn: 3
        }
    ]
}

const addEditDelete = new AddEditDeletePopup(personsApiUrl, personsInputFields);

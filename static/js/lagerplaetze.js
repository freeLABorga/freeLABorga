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

// Konstanten
const placesApiUrl = "api/lagerplatz"
const linkPlaces = "gegenstaende.html?place=";

function showAddPersonPopup(e) {
    addEditDelete.showAddPopup();
}

document.getElementById("button-add-place").addEventListener("click", showAddPersonPopup);


function showDeletePlacePopup(e) {
    addEditDelete.showDeletePopup(e);
}

function showEditPlacePopup(e) {
    addEditDelete.showEditPopup(e);
}

function showItemsWithPlace(e) {
    let rowName = null;
    if (e.target.tagName == "TD")
        rowName = e.target.parentElement.firstChild.innerText;
    else if (e.target.tagName == "TR")
        rowName = e.target.firstChild.innerText;

    if (rowName != null)
        location.href = linkPlaces + rowName;
}

const placesFields = {
    idField: "id",
    eventListener: showItemsWithPlace,
    cells: [
        {
            jsonName: "name"
        },
        {
            jsonName: "number"
        },
        {
            cellClass: "edit-place-button",
            staticContent: "<i class='fa-solid fa-pen-to-square'></i>",
            eventListener: showEditPlacePopup
        },
        {
            cellClass: "delete-place-button",
            staticContent: "<i class='fa-solid fa-trash-can'></i>",
            eventListener: showDeletePlacePopup
        }

    ]
};

const t = new LoadTableFromJson(placesApiUrl, placesFields, false);


const placesInputFields = {
    popupTitleAdd: "Lagerplatz hinzufügen",
    popupTitleEdit: "Lagerplatz bearbeiten",
    popupTitleDelete: "Lagerplatz löschen",
    popupMessageDelete: "<p>Möchten Sie den Lagerplatz <strong>{1}</strong> wirklich löschen?</p>",
    getDataUrl: placesApiUrl,
    eventAfterAction: () => t.init(),
    fields: [
        {
            element: "input",
            displayName: "Name",
            jsonName: "name",
            type: "text",
            required: true,
            tableColumn: 0
        }
    ]
};

const addEditDelete = new AddEditDeletePopup(placesApiUrl, placesInputFields);

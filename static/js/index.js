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

import { LoadTableFromJson } from "./load_table_from_json.js";

const thingsOverviewApiUrl = "api/gegenstand_uebersicht";
const allThingsViewUrl = "gegenstaende.html?item_name=";
const getPersonToAnonymizeUrl = "api/person_to_anonymize";
const excludeFromAnonymisationUrl = "api/person/{matr_nr}/exclude_from_anonymisation";
const executeAnonymisationUrl = "api/person/{matr_nr}";
const anonymizePopupTitle = "Person anonymisieren?";
const anonymizePopupMessage = "<p>Möchten Sie die Person <strong>{0} - {1}, {2}</strong> anonymisieren?</p><p>Sie wurde seit über {days} Tagen nicht mehr benutzt.</p><p>Dadurch wird die persönlichen Daten gelöscht. Nur die Matrikel-Numer bei Ausleihen bleibt erhalten.";
const errorMessageAnonymize = "Fehler beim Anonymisieren der Person";

function showItems(e) {
    let rowName = null;
    if (e.target.tagName == "TD") 
        rowName = e.target.parentElement.dataset.id;
    else if (e.target.tagName == "TR")
        rowName = e.target.dataset.id;

    if (rowName != null)
        location.href = allThingsViewUrl + rowName;
}

const itemsFields = {
    idField: "name",
    eventListener: showItems,
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
            jsonName: "total"
        },
        {
            jsonName: "available",
        },
        {
            seperator: " / ",
            jsonName0: "available",
            jsonName1: "total",
        },
    ]
}


const t = new LoadTableFromJson(thingsOverviewApiUrl, itemsFields, false);


function getPersonToAnonymize() {
    fetch(getPersonToAnonymizeUrl)
        .then((response) => response.json())
        .then((jsonDoc) => {
            if (jsonDoc.personAvailable === true) {
                let msg = anonymizePopupMessage
                    .replace("{0}", Object.values(jsonDoc.person)[0])  // Ersetzt die Platzhalter {0}, {1} und {2} durch die jeweiligen echten Werte
                    .replace("{1}", Object.values(jsonDoc.person)[1])
                    .replace("{2}", Object.values(jsonDoc.person)[2])
                    .replace("{days}", jsonDoc.days)
                    .concat(`<div class="error-message" id="popup-error-message" style="display: none;"></div>`);
                new ConfirmationPopup((state, popup) => anonymizePersonAction(jsonDoc.person.matrNr, state, popup), anonymizePopupTitle, msg, false, false)
            }
        });
}

async function anonymizePersonAction(id, state, popup) {
    const errorMessageElement = document.getElementById("popup-error-message");

    const fetchParams = {
        method: 'DELETE', 
    }

    const url = (state ? executeAnonymisationUrl : excludeFromAnonymisationUrl).replace("{matr_nr}", id)
    const response = await sendData(url, fetchParams, errorMessageAnonymize);

    if (!response.success) {
        errorMessageElement.innerHTML = response.message;
        errorMessageElement.style.display = "block";
    } else {
        popup.close();
    }
}

/**
 * Sendet daten an den Server. Gibt im Fehlerfall eine Meldung zurück.
 * @param {string} url Url, zu der die Anfrage gesendet werden soll.
 * @param {RequestInit} fetchParams Parameter, die der Fetch-Funktion mitgegeben werden.
 * @param {string} errorText Text, der im Fehlerfall zurückgebeben wird.
 * @returns Objekt mit {success: boolean, message: string}
 */
async function sendData(url, fetchParams, errorText) {
    try {
        const response = await fetch(url, fetchParams);

        if (!response.ok) {
            return {
                success: false,
                message: `${errorText}<br>Fehler ${response.status}: ${response.statusText}`
            };
        }

        return await response.json();

    } catch (e) {
        return {
            success: false,
            message: `<p>${errorText}<br>${e}</p>`
        };
    }
}

getPersonToAnonymize();

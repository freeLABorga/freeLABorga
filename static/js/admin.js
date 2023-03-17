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

// Konstanten
const adminApiUrl = "api/admin"
const errorMessageLoadingData = "Fehler beim Laden der Daten.<br>Fehler ";
const errorMessageInvalidNumber = "Ungültige Zahl eingegeben.";
const errorMessageProcessingData = "Fehler beim Verarbeiten der Daten.";
const errorMessageStoreData = "Fehler beim Speichern.";
const successMessage = "Daten erfolgreich gespeichert.";
const errorTitle = "Fehler";
const resultElementId = "result-message";
const errorMessageClass = "error-message";
const successMessageClass = "success-message";

/** 
 * Lädt die Admin-Seiten-Daten und gibt sie weiter an showAdminData()
 */
async function loadAdminData() {
    try {
        const response = await fetch(adminApiUrl);
        if (!response.ok) {
            new ConfirmationPopup(() => {}, errorTitle, `<p>${errorMessageLoadingData}${response.status}: ${response.statusText}</p>`, true);
        } else {
            const jsonDoc = await response.json();
            showAdminData(jsonDoc);
        }

    } catch (e) {
        new ConfirmationPopup(null, errorTitle, `<p>${errorMessageProcessingData}<br>${e}</p>`, true);
    }
}


/**
 * Zeigt die übergebenen Daten auf der Seite in den entsprechenden Input-Feldern an
 */
function showAdminData(jsonDoc) {
    if (jsonDoc.hasOwnProperty("imprintText"))
        document.getElementById("imprint-text").innerHTML = jsonDoc.imprintText;

    if (jsonDoc.hasOwnProperty("privacyText"))
        document.getElementById("privacy-text").innerHTML = jsonDoc.privacyText;
    
    if (jsonDoc.hasOwnProperty("imprintHtml") && jsonDoc.imprintHtml === true) 
        document.getElementById("imprint-html-active").checked = true;
        
    if (jsonDoc.hasOwnProperty("privacyHtml") && jsonDoc.privacyHtml === true) 
        document.getElementById("privacy-html-active").checked = true;

    if (jsonDoc.hasOwnProperty("itemsPerPage")) 
        document.getElementById("items-per-page").value = jsonDoc.itemsPerPage;

    if (jsonDoc.hasOwnProperty("daysUntilAnonymize")) 
        document.getElementById("anonymisation-days").value = jsonDoc.daysUntilAnonymize;
}


/** 
 * Sendet die Daten von den Input-Feldern an das Backend
 */
async function submitAdminForm() {
    // Wenn Eingaben ungültig, Speichern abbrechen
    if (!validateInput("items-per-page") || !validateInput("anonymisation-days")) {
        return;
    }

    let jsonDoc = {};

    jsonDoc.imprintText = document.getElementById("imprint-text").value;
    jsonDoc.privacyText = document.getElementById("privacy-text").value;
    jsonDoc.imprintHtml = document.getElementById("imprint-html-active").checked;
    jsonDoc.privacyHtml = document.getElementById("privacy-html-active").checked;
    jsonDoc.itemsPerPage = parseInt(document.getElementById("items-per-page").value);
    jsonDoc.daysUntilAnonymize = parseInt(document.getElementById("anonymisation-days").value);

    const fetchParams = {
        method: 'PUT', 
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify(jsonDoc)
    }
 
    try {
        const response = await fetch(adminApiUrl, fetchParams);

        if (!response.ok) {
            displayMessage(false, `<p>${errorMessageStoreData}<br>${response.status}: ${response.statusText}</p>`);
        } else {
            const jsonResult = await response.json();
            const message = jsonResult.success ? successMessage : jsonResult.message
            displayMessage(jsonResult.success, message);
        }

    } catch (e) {
        displayMessage(false, `<p>${errorMessageStoreData}<br>${e}</p>`);
    }

}

/**
 * Zeigt eine Ergebnismeldung an
 * @param {boolean} success Aktion zuvor erfolgreich?
 * @param {String} msg Nachricht, die angezeigt werden soll
 */
function displayMessage(success, msg) {
    const resultElement = document.getElementById(resultElementId);
    if (success) {
        resultElement.classList.remove(errorMessageClass);
        resultElement.classList.add(successMessageClass);
    } else  {
        resultElement.classList.remove(successMessageClass);
        resultElement.classList.add(errorMessageClass);
    }
    resultElement.style.display = "block";
    resultElement.innerHTML = msg;
}

/**
 * Überprüft, ob eine Eingabe gültig ist 
 * und zeigt im Fehlerfall eine entsprechende Meldung
 * @param elementId ID des zu überprüfenden Eingabefelds
 * @returns Boolean, je nachdem ob Eingabe gültig
 */
function validateInput(elementId) {
    let inputElement = document.getElementById(elementId);
    if (!inputElement.validity.valid) {
        displayMessage(false, errorMessageInvalidNumber);
        return false;
    }
    return true;
}


loadAdminData();
document.getElementById("save-admin-texts-button").addEventListener("click", submitAdminForm);

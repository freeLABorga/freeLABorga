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

const errorMessageLoadingData = "Keine Daten empfangen.";
const errorMessageProcessData = "<p>Fehler beim Verarbeiten der Daten.</p>";
const errorTitleProcessData = "FEHLER";
const placeholderRequired = "Erforderlich";
const placeholderMultipleValues = "Mehrere kommagetrennte Werte";
const placeholderPrice = "X,XX";
const errorMessageFieldEmpty = "Erforderliches Feld leer";
const errorMessageInvalidEmail = "E-Mail Adresse ungültig";
const errorMessageInvalidPrice = "Preis fehlerhaft (Format x,xx erforderlich)";
const errorMessageAdd = "Fehler beim Hinzufügen.";
const errorMessageEdit = "Fehler beim Bearbeiten.";
const errorMessageDelete = "Fehler beim Löschen.";
const idUrlSuffix = "?id=";
const idOfIdCell = "Textfeld_ID";

export class AddEditDeletePopup {
    /**
     * Erstellt Popups zum hinzufügen, bearbeiten und Löschen von Elementen
     * und zeigt diese bei speziellen Events an.
     * @param {String} jsonUrl URL, zu der die Anfragen gesendet weden.
     * @param {Object} fields Objekt mit allen Konfigurationswerten.
     */
    constructor(jsonUrl, fields) {
        this.valueSeperator = ", ",
        this.jsonUrl = jsonUrl;
        this.fields = fields;
        this.id = undefined;

        // Option-List laden (Textvorschläge)
        if (fields.hasOwnProperty("optionListUrl")) {
            fetch(fields.optionListUrl)
                .then((response) => response.json())
                .then((jsonDoc) => this.optionLists = jsonDoc);
        }
    }

    getId(e) {
        // ID in Detailansicht aus Zelle lesen
        const detailViewIdElement = document.getElementById(idOfIdCell);

        // Zeile (TR) kann unterschiedliche Entfernung von geklicktem Element haben
        let rowElement = e.target.parentElement.parentElement;
        if (rowElement.tagName != "TR") {
            rowElement = rowElement.parentElement;
        }

        // ID aus Dataset in jeweiliger Zeile lesen
        if (rowElement.tagName == "TR" && rowElement.hasAttribute("data-id")) {
            return rowElement.dataset.id;
        } else if (detailViewIdElement != null) {
            return detailViewIdElement.innerText;
        }
        return null;
    }

    showAddPopup() {
        this.showPopupForm((o) => this.addItem(o));
    }

    showEditPopup(e) {
        const id = this.getId(e);
        if (id == null) return;
        fetch(this.fields.getDataUrl + idUrlSuffix + id)
            .then((response) => response.json())
            .then((jsonDoc) => {if (jsonDoc.data[0] == null) throw new Error(errorMessageLoadingData); else return jsonDoc})
            .then((jsonDoc) => this.showPopupForm(o => this.editItem(o), jsonDoc.data[0], id))
            .catch(e => new ConfirmationPopup(null, errorTitleProcessData, errorMessageProcessData + e, true));
    }

    showDeletePopup(e) {
        let id = this.getId(e);
        if (id == null) return;
        const url = this.fields.getDataUrl + idUrlSuffix + id;
        fetch(url)
            .then((response) => response.json())
            .then((jsonDoc) => {
                if (jsonDoc.data[0] == null) {
                    throw new Error(errorMessageLoadingData);
                }
                const message = this.fields.popupMessageDelete
                    .replace("{0}", Object.values(jsonDoc.data[0])[0])  // Ersetzt die Platzhalter {0}, {1} und {2} durch die jeweiligen echten Werte
                    .replace("{1}", Object.values(jsonDoc.data[0])[1])
                    .replace("{2}", Object.values(jsonDoc.data[0])[2])
                    .concat(getErrorMessageField().outerHTML);
                new ConfirmationPopup((state, popup) => this.deleteItem(id, state, popup), this.fields.popupTitleDelete, message, false, false);
            })
            .catch(e => new ConfirmationPopup(null, errorTitleProcessData, errorMessageProcessData + e, true));
    }


    /**
     * Stellt Eingabeformular zusammen.
     * @param postFunction Funktion, die nach Formularbestätigung aufgerufen wird
     * @param preValues Default-Werte
     * @param id ID des Objekts
     */
    showPopupForm(postFunction, preValues={}, id=undefined) {
        this.id = id;
        let body = document.createElement("div");
        this.fields.fields.forEach(fieldConf => {
            const inputRowElement = document.createElement("div");
            inputRowElement.className = "input-row";
            const labelElement = document.createElement("label");
            labelElement.setAttribute("for", fieldConf.jsonName);
            labelElement.innerText = fieldConf.displayName + ": ";
            const inputElement = document.createElement(fieldConf.element);
            inputElement.id = fieldConf.jsonName;
            
            if (fieldConf.hasOwnProperty("required") && fieldConf.required === true) {
                inputElement.placeholder = placeholderRequired;
            }
            
            if (fieldConf.type == "date") {
                inputElement.setAttribute("value", getFormattedDate()); 
            }
            
            if (fieldConf.type == "itemId") {
                inputElement.setAttribute("value", document.getElementById("Textfeld_ID").innerHTML);
                inputRowElement.style.display = "none";
            }

            inputElement.type = fieldConf.type;
            
            if (fieldConf.type == "price") {
                inputElement.setAttribute("min", "0.00"); 
                inputElement.setAttribute("step", "0.01"); 
                inputElement.setAttribute("placeholder", placeholderPrice);
                inputElement.type = "number";
            }


            this.addOptionListToInputField(inputElement, fieldConf);

            
            if (preValues.hasOwnProperty(fieldConf.jsonName)) {
                let value = preValues[fieldConf.jsonName];
                if (Array.isArray(preValues[fieldConf.jsonName])) {
                    value = value.join(this.valueSeperator);
                }

                if (fieldConf.type == "checkbox"){
                    inputElement.checked = value;
                } else {
                    inputElement.setAttribute("value", value);
                }
            }
            
            inputRowElement.appendChild(labelElement);
            inputRowElement.appendChild(inputElement);
            body.appendChild(inputRowElement);
        });

        body.appendChild(getErrorMessageField());

        // Titel des Popups: Hinzufügen oder Bearbeiten
        const popupTitle = (Object.keys(preValues).length === 0) ? this.fields.popupTitleAdd : this.fields.popupTitleEdit

        const classThis = this;
        const popup = new ConfirmationPopup((state, popup) => classThis.checkInputs(state, popup, postFunction), popupTitle, body, true, false);

    }


    /**
     * Fügt zu dem übergebenen Input-Feld eine Vorschlagsliste hinzu, falls diese Liste existiert.
     * @param {HTMLElement} inputElement Eingabefeld
     * @param {Object} fieldConf Konfigurationsobjekt für dieses Feld
     */
    addOptionListToInputField(inputElement, fieldConf) {
        if (this.hasOwnProperty("optionLists") && this.optionLists.hasOwnProperty(fieldConf.jsonName)) {
            const datalistId = fieldConf.jsonName + "-datalist";
            inputElement.setAttribute("list", datalistId);

            const datalistElement = document.createElement("datalist");
            datalistElement.id = datalistId;

            // Vorschläge für Eingabe hinzufügen
            this.refreshOptionListValues(inputElement, fieldConf, datalistElement)
            
            document.getElementsByTagName("body")[0].appendChild(datalistElement);

            if (fieldConf.hasOwnProperty("multiple") && fieldConf.multiple === true) {
                inputElement.setAttribute("placeholder", placeholderMultipleValues);
                inputElement.setAttribute("multiple", true);
    
                inputElement.addEventListener("input", () => this.refreshOptionListValues(inputElement, fieldConf, datalistElement));
                
            }
        }
    }

    /**
     * Aktualisiert die Vorschläge des Eingabefeld-Elements.
     * Besonders nützlich, wenn mehrere Werte eingegeben werden können.
     * @param inputElement Eingabe-Element
     * @param fieldConf Konfigurations-Objekt des Eingabe-Elementes
     * @param datalistElement Objekt, in der die Daten vom Datalist-Element gespeichert werden
     */
    refreshOptionListValues(inputElement, fieldConf, datalistElement) {
        let alreadyInserted = inputElement.value.substring(0, inputElement.value.lastIndexOf(this.valueSeperator[0]));
        if (alreadyInserted != "") {
            alreadyInserted += this.valueSeperator;
        }

        datalistElement.innerHTML = ""; // Datalist leeren
        this.optionLists[fieldConf.jsonName].forEach(optionValue => {
            const optionElement = document.createElement("option");
            optionElement.value = alreadyInserted + optionValue;
            datalistElement.appendChild(optionElement);
        });
    }

    /** Prüft, ob alle Eingaben valide sind und führt im Erfolgsfall eine übergebene Funktion aus.
     *  Andernfalls wird eine Fehlermeldung im Popup angezeigt
     * @param state Boolean, ob die Daten gesendet werden sollen oder abgebrochen wurde.
     * @param popup Popup-Element, in das die Daten eingegeben wurde.
     * @param postFunction Funktion, die im Erfolgsfall nach der Validierung ausgeführt werden soll.
     */
    async checkInputs(state, popup, postFunction) {
        // Wenn abgebrochen wurde, Popup einfach schließen
        if (!state) {
            popup.close();
            return;
        }

        const errorMessageElement = document.getElementById("popup-error-message");
        errorMessageElement.style.display ="none";
        let response = {};
        let inputFieldError = 0;
        this.fields.fields.forEach(fieldConf => {
            const inputElement = document.getElementById(fieldConf.jsonName);
            response[fieldConf.jsonName] = inputElement.value;

            if (inputElement.hasAttribute("multiple")) {
                response[fieldConf.jsonName] = inputElement.value.split(this.valueSeperator[0]);
            }
            if (fieldConf.type == "price"){
                if(inputElement.value == ""){
                    inputElement.value = 0.00;
                }
                response[fieldConf.jsonName] = parseFloat(inputElement.value);
            }
            if (fieldConf.type == "checkbox"){
                response[fieldConf.jsonName] = inputElement.checked;
            }

            if (fieldConf.hasOwnProperty("required") && fieldConf.required === true && inputElement.value == "") {
                errorMessageElement.innerText = errorMessageFieldEmpty;
            } else if (fieldConf.type == "email" && inputElement.value != "" && !validateEmail(inputElement.value)) {
                errorMessageElement.innerText = errorMessageInvalidEmail;
            } else if (fieldConf.type == "price" && !inputElement.validity.valid) {
                errorMessageElement.innerText = errorMessageInvalidPrice;
            } else {
                inputElement.classList.remove("input-field-error");
                return;
            }
            inputFieldError++;
            inputElement.classList.add("input-field-error");
            errorMessageElement.style.display = "block";
        })
        
        
        if (inputFieldError == 0) {
            const result = await postFunction(response);
            
            if (!result.success) {
                errorMessageElement.innerHTML = result.message;
                errorMessageElement.style.display = "block";
            } else {
                popup.close();
                this.eventAfterAction();
            }
        }
    }
    
    
    async addItem(object) {
        const fetchParams = {
            method: 'POST', 
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(object)
        }
        
        return await sendData(this.jsonUrl, fetchParams, errorMessageAdd);
    }

    async editItem(object) {
        const fetchParams = {
            method: 'PUT', 
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(object)
        }
        const url = this.jsonUrl + "/" + this.id;
        return await sendData(url, fetchParams, errorMessageEdit);
    }

    async deleteItem(id, state, popup) {
        if (!state) {
            popup.close();
            return;
        }

        const errorMessageElement = document.getElementById("popup-error-message");

        const fetchParams = {
            method: 'DELETE', 
        }

        const url = this.jsonUrl + "/" + id;
        const response = await sendData(url, fetchParams, errorMessageDelete);

        if (!response.success) {
            errorMessageElement.innerHTML = response.message;
            errorMessageElement.style.display = "block";
        } else {
            popup.close();
            this.eventAfterAction();

            //Fall: Gegenstand wird ind Detailansicht gelöscht -> umleiten auf Gegenstände-Seite
            if (window.location.href.includes("detailansicht.html") && this.fields.hasOwnProperty("deleteAndSwitch")) {
                window.location.assign("gegenstaende.html");
            }
        }
    }

    async eventAfterAction() {
        if (this.fields.hasOwnProperty("eventAfterAction")) {
            this.fields.eventAfterAction();
        }
    }

}

/**
 * Prüft, ob übergebener String eine gültige E-Mail Adresse ist
 * @param {string} input E-Mail-Adresse, die validiert werden soll.
 * @returns Boolean, ob valide E-Mail Adresse übergeben wurde.
 */
function validateEmail(input) {
    return /^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w+)+$/.test(input);
}

/**
 * Quelle: https://isotropic.co/how-to-format-a-date-as-dd-mm-yyyy-in-javascript/
 * @returns Aktuelles Datum als String in Form dd-mm-yyy
 */
function getFormattedDate() {
    const inputDate = new Date();
    let date, month, year;
  
    date = inputDate.getDate();
    month = inputDate.getMonth() + 1;
    year = inputDate.getFullYear();
  
      date = date
          .toString()
          .padStart(2, '0');
  
      month = month
          .toString()
          .padStart(2, '0');
  
    return `${year}-${month}-${date}`;
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
            message: `${errorText}<br>${e}`
        };
    }
}

/**
 *  Erstellt ein DIV-Element für mögliche Fehlermeldungen
 * @returns Dieses DIV-Element für Fehlermeldungen
 */ 
function getErrorMessageField() {
    const errorMessageField = document.createElement("div");
    errorMessageField.classList.add("error-message");
    errorMessageField.id = "popup-error-message";
    errorMessageField.style.display = "none";
    return errorMessageField;
}

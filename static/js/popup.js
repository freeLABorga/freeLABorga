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


// Konstanten
const textButtonYes = "JA";
const textButtonNo = "NEIN";
const textButtonOK = "OK";
const textButtonCancel = "ABBRECHEN";

/**
 * Klasse zum erstellen eines Bestätigungs-Popups
 * 
 */
class ConfirmationPopup {
    /**
     * Erstellt ein Popup
     * @param fct Funktion, die nach bestätigen / schließen des Popups aufgerufen werden soll.
     * @param headerText Text, der in der Kopfzeile des Popups stehen soll.
     * @param bodyText HTML (als String oder HTMLElement), das im Popup angezeigt werden soll.
     * @param allowExit Boolean, ob das Popup nur mit den unteren Buttons geschlossen werden darf.
     * @param autoClose Boolean, ob das Popup automatisch oder durch die aufgerufene Funktion geschlossen werden soll.
     */
    constructor(fct, headerText, bodyText, allowExit, autoClose = true) {
        this.fct = fct;
        this.hText = headerText;
        this.bText = bodyText;
        this.allowExit = allowExit;
        this.autoClose = autoClose;
        this.click = {handleEvent:this.confirmPopup, self:this};
        this.key = {handleEvent:this.keyEventListener, self:this};

        this.openPopup();
    }
    
    openPopup() {
        this.elementOverlay = document.getElementById("overlay");
        this.elementPopup = document.getElementById("confirmation-popup");
        this.elementPopupClose = document.getElementById("confirmation-popup-close");
        this.elementButtonYes = document.querySelector("#pupup-confirm-buttons .button-yes");
        this.elementButtonNo = document.querySelector("#pupup-confirm-buttons .button-no");

        this.elementOverlay.style.display = "block";
        this.elementPopup.getElementsByTagName("H3")[0].innerText = this.hText;
        if (typeof this.bText === "string") {
            this.elementPopup.getElementsByClassName("popup-content")[0].innerHTML = this.bText;
        } else {
            this.elementPopup.getElementsByClassName("popup-content")[0].innerHTML = "";
            this.elementPopup.getElementsByClassName("popup-content")[0].appendChild(this.bText);
        }
        this.elementPopup.style.display = "block";

        this.elementButtonYes.addEventListener("click", this.click);
        this.elementButtonNo.addEventListener("click", this.click);
        
        if (this.allowExit) {
            this.elementPopupClose.style.display = "block";
            this.elementPopupClose.addEventListener("click", this.click);
            this.elementOverlay.addEventListener("click", this.click);
            this.elementButtonYes.innerText = textButtonOK;
            this.elementButtonNo.innerText = textButtonCancel;
        } else {
            this.elementPopupClose.style.display = "none";
            this.elementButtonYes.innerText = textButtonYes;
            this.elementButtonNo.innerText = textButtonNo;
        }

        this.focusInputField();

        document.addEventListener('keydown', this.key);
    }
    
    keyEventListener(e) {
        const classThis = this.self;
        if (e.key == "Escape") classThis.confirmPopup(false);
        else if (e.key == "Enter") classThis.confirmPopup(true);
        
    }
    
    focusInputField() {
        const inputField = this.elementPopup.getElementsByTagName("input")[0];
        if (inputField != null && inputField.type != "date") {
            inputField.focus();
            inputField.setSelectionRange(0, inputField.value.length);
        }
    }
    
    confirmPopup(e) {
        let classThis = this;
        if (this.hasOwnProperty("self")) 
            classThis = this.self;
        
        // Nur, wenn von EventListener aufgerufen, wird Popup nicht geschlossen
        if (classThis.autoClose && this.hasOwnProperty("self")) {
            classThis.close();
        }

        // Zurückgeben, welcher Button geklickt wurde
        let result = (e === true || (e.target !== undefined && e.target.className == "button-yes"));
        classThis.fct(result, classThis);
    }

    close() {
        let classThis = this;
        if (this.hasOwnProperty("self")) 
            classThis = this.self;

        classThis.elementOverlay.style.display = "none";
        classThis.elementPopup.style.display = "none";
        
        classThis.elementButtonNo.removeEventListener("click", classThis.click);
        classThis.elementButtonYes.removeEventListener("click", classThis.click);
        classThis.elementPopupClose.removeEventListener("click", classThis.click);
        classThis.elementOverlay.removeEventListener("click", classThis.click);

        document.removeEventListener('keydown', classThis.key);
    }

}

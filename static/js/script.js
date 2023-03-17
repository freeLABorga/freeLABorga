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

// Define constants
const textNotFound = "Nichts gefunden";
const urlThings = "gegenstaende.html";
const urlPersons = "personen.html";
const urlDetailView = "detailansicht.html";
const searchUrlSuffix = "?search=";
const idUrlSuffix = "?id=";
const urlCategory = "gegenstaende.html?cat=";
const urlPlace = "gegenstaende.html?place=";
const urlCategoriesPlaces = "api/kategorie_lagerplatz";
const urlQuickSearchThingsPersons = "api/schnellsuche?search=";
const errorMessageLoadingSuggestions = "Fehler beim Laden der Vorschläge";


/**
 * Öffnet oder schließt das mobile Menü
 */
function toogleMobileMenu() {
    const nav = document.getElementsByTagName("nav")[0];

    // if menu is disabled, activate it; else disable it
    nav.classList.toggle("active");

    if (nav.classList.contains("active")) {
        nav.innerHTML = `
    <ul>
      <li><a href="gegenstaende.html">Gegenstände</a></li>
      <li><a href="lagerplaetze.html">Lagerplätze</a></li>
      <li><a href="heatmap.html">Heatmap</a></li>
      <li><a href="personen.html">Personen</a></li>
      <li><a href="admin.html">Admin-Bereich</a></li>
    </ul>`;
        // open the mobile menu, if class 'active' is set
        // to do that, calculate the hight of the menu list
        const height = nav.getElementsByTagName("ul")[0].offsetHeight + "px";
        nav.style.maxHeight = height;

        // Close search input
        document.getElementById("search").classList.remove("active");
        closeSearchPopup();
    } else {
        // close the mobile menu, if class 'active' is not set 
        nav.style.maxHeight = null;
        nav.innerHTML = `<ul>
    <li>
        <a href="#">Inventar</a>
        <ul class="dropdown-menu">
            <li><a href="gegenstaende.html">Gegenstände</a></li>
            <li><a href="lagerplaetze.html">Lagerplätze</a></li>
        </ul>
    </li>
    <li><a href="heatmap.html">Heatmap</a></li>
    <li><a href="personen.html">Personen</a></li>
    <li><a href="admin.html">Admin-Bereich</a></li>
</ul>`;
    }
}



// ------------------ Search-Popup displaying logic ------------------
const searchInput = document.getElementById("search-input");

/**
 * Öffnet das Suchpopup und fügt die Event-Listener hinzu.
 */
function openSearchPopup(e) {
    document.getElementById("search-popup").style.display = "block";
    document.getElementById("overlay").style.display = "block";
    document.getElementById("button-search").classList.add("active");
    updateSearchResults(e);


    // Add event listener only if not already done
    if (!searchInput.hasAttribute("event-listener")) {
        searchInput.setAttribute("event-listener", "event-listener");

        document.getElementById("overlay").addEventListener('click', closeSearchPopup);
        document.getElementById("search-popup").addEventListener('keydown', activateSearchInput);

        searchInput.addEventListener('input', updateSearchResults);
        searchInput.addEventListener('keydown', e => {
            if (e.key == "Escape") closeSearchPopup(e);
            else if (e.key == "Enter") fullTextSearch()
        });
    }
}
document.getElementById("menu-button").addEventListener('click', toogleMobileMenu);
document.getElementById("search-input").addEventListener('focus', openSearchPopup);
document.getElementById("things-search").addEventListener("click", e => fullTextSearch(urlThings));
document.getElementById("persons-search").addEventListener("click", e => fullTextSearch(urlPersons));
document.getElementById("button-search").addEventListener("click", e => fullTextSearch());

/**
 * Führt Volltextsuche aus mit Suchstring, der im Suchfeld steht
 * @param url URL, von der die Volltextsuche ausgeführt werden soll (optional)
 */
function fullTextSearch(url = undefined) {
    // Wenn keine URL übergeben wurde, in Gegenständen suchen.
    if (url == undefined) {
        url = window.location.href.split('?')[0];
        if (!url.includes(urlPersons)) {   // außer wenn aktuelle Seite "personen.html" ist
            url = urlThings;
        }
    }
    // Sonderzeichen maskieren
    const completeUrl = url + searchUrlSuffix + encodeURIComponent(searchInput.value);
    window.location.href = completeUrl;
}

/**
 * Schließt das Suchpopup und entfernt die Event-Listener.
 */
function closeSearchPopup(e) {
    searchInput.removeAttribute("event-listener");

    document.getElementById("search-popup").style.display = "none";
    document.getElementById("overlay").style.display = "none";
    document.getElementById("button-search").classList.remove("active")

    document.getElementById("overlay").removeEventListener('click', closeSearchPopup);
    document.getElementById("search-popup").removeEventListener('keydown', activateSearchInput);

    searchInput.removeEventListener('input', updateSearchResults);
}


// Array für alle Kategorien und Lagerplätze (wird später durch fetch gefüllt)
let categories = [];
let places = []
getCategoriesPlaces();


/**
 * Namen der Kategorien und Lagerplätze aus JSON-Dokument laden
 */
function getCategoriesPlaces() {
    fetch(urlCategoriesPlaces)
        .then((response) => response.json())
        .then((jsonDoc) => {
            categories = jsonDoc.categories;
            places = jsonDoc.place;
        });
}

/**
 * Aktualisiert die Suchvorschläge 
 * (Gegenstände, Personen, Kategorien, Gegenstände)
 * @param {Event} e Event, das durch Tippen im Eingabefeld ausgelöst wird.
 */
function updateSearchResults(e) {
    const text = e.target.value;
    filterCategories(text);
    filterPlaces(text);
    updateThingsPersonsResults(text);
}

/**
 * Aktualisiert die Vorschläge für Gegenstände und Personen gemäß dem übergebenen Suchstring.
 * @param {String} text Text, nach dem gesucht werden soll.
 */
function updateThingsPersonsResults(text) {
    const searchUrl = urlQuickSearchThingsPersons + text;
    const thingsSuggestionList = document.getElementById("things-suggestion-list");
    const personsSuggestionList = document.getElementById("persons-suggestion-list");
    fetch(searchUrl)
        .then((response) => response.json())
        .then((jsonDoc) => {
            fillSuggestionList(thingsSuggestionList, jsonDoc.things, urlDetailView);
            fillSuggestionList(personsSuggestionList, jsonDoc.persons, urlPersons);
        })
        .catch(e => {
            thingsSuggestionList.innerText = errorMessageLoadingSuggestions;
            personsSuggestionList.innerText = errorMessageLoadingSuggestions;
        });
}

/**
 * Füllt die übergebene HTML-Liste mit den Objekten aus dem übergebenen Array.
 * @param {Element} htmlListElement HTML-Element, in das die Listenelemente eingefügt werden.
 * @param {Array} objectList Array, in dem Objekte mit ID und Name gespeichert sind.
 * @param {String} link Link-Anfang für die Listenelemente (hieran wird die jeweilige ID angehängt.)
 */
function fillSuggestionList(htmlListElement, objectList, link) {
    htmlListElement.innerHTML = "";
    objectList.forEach(element => {
        const listItem = document.createElement("li");
        const listItemLink = document.createElement("a");
        listItemLink.href = link + idUrlSuffix + element.id;
        listItemLink.innerText = element.id + " - " + element.name;
        listItem.appendChild(listItemLink);
        htmlListElement.appendChild(listItem);
    });

}

/**
 * Filtert die Kategorien im Suchpopup nach dem übergebenen Text
 */
function filterCategories(text) {
    const categoryFilter = document.getElementById("category-filter");
    filterInSearchPopup(categories, text, categoryFilter, urlCategory);
}

/**
 * Filtert die Lagerplätze im Suchpopup nach dem übergebenen Text
 */
function filterPlaces(text) {
    const placesFilter = document.getElementById("place-filter");
    filterInSearchPopup(places, text, placesFilter, urlPlace);
}

/**
 * Filtert die Elemente nach dem übergebenen Text
 * @param {Array} allValues Array mit allen Werten, die gefiltert werden sollen.
 * @param {String} searchValue String nach dem gesucht wird
 * @param {Element} section HTML-Element, in dem die gefilterten Elemente angezeigt werden sollen.
 * @param {String} url Link-Anfang für die Elemente (hieran wird der jeweilige Name angehängt.)
 */
function filterInSearchPopup(allValues, searchValue, section, url) {
    section.querySelectorAll("a, p").forEach(element => section.removeChild(element));
    let itemFound = false;
    allValues.sort().forEach(item => {
        if (item.toLocaleLowerCase().includes(searchValue.toLocaleLowerCase())) {
            itemFound = true;
            const categoryElement = document.createElement("a");
            categoryElement.href = url + item;
            categoryElement.classList.add("cat-item");
            categoryElement.innerText = item;
            section.append(categoryElement);
        }
    })
    if (!itemFound) {
        const notFoundText = document.createElement("p");
        notFoundText.innerText = textNotFound;
        section.append(notFoundText);
    }
}

/**
 * Aktiviert das Suchpopup, falls gewisse Tasten im Sucheingabefeld gedrückt wurden.
 */
function activateSearchInput(e) {
    const controlKeys = ["ArrowDown", "ArrowUp", "ArrowLeft", "ArrowRight", "Tab", "PageDown", "PageUp", "Control", "Shift", "Alt"];
    if (!controlKeys.includes(e.key)) {
        const searchInput = document.getElementById("search-input");
        searchInput.focus();
    }
}

/**
 * Öffnet oder schließt das Suchfeld in der mobilen Ansicht.
 */
function toogleMobileSearchInput(e) {
    const search = document.getElementById("search");
    // if search input is disabled, activate it; else disable it
    search.classList.toggle("active");
    closeSearchPopup()
    document.getElementById("search-input").focus();
}

document.getElementById("mobile-search-button").addEventListener('click', toogleMobileSearchInput);

/**
 * Extrahiert Suchstring aus den GET-Parametern und zeigt ihn im Suchfeld an.
 */
function showSearchString() {
    const searchValue = new URLSearchParams(window.location.search).get("search");
    searchInput.value = searchValue;

}

showSearchString();

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

const imprintPageUrl = "/impressum.html";
const imprintApiUrl = "/api/impressum"

const privacyPageUrl = "/datenschutz.html";
const privacyApiUrl = "/api/datenschutz"

const contentElementName = "inner-content";

const path = window.location.pathname;

if (path == imprintPageUrl) {
    getContent(imprintApiUrl)
} else if (path == privacyPageUrl) {
    getContent(privacyApiUrl)
}

function getContent(url) {
    fetch(url)
    .then(response => response.text())
    .then(text => {
        const contentElement = document.getElementById(contentElementName);
        contentElement.innerHTML = text;
    })
}

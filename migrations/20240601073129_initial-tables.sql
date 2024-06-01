-- This file is part of Hakobiya.
--
-- Hakobiya is free software: you can redistribute it and/or modify it under the terms of
-- the GNU Affero General Public License as published by the Free Software Foundation, either
-- version 3 of the License, or (at your option) any later version.
--
-- Hakobiya is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
-- without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
-- See the GNU Affero General Public License for more details.
--
-- You should have received a copy of the GNU General Public License along with Hakobiya.
-- If not, see <https://www.gnu.org/licenses/>.
--
CREATE TABLE users (
    name    VARCHAR(255) NOT NULL,
    mail    VARCHAR(255) NOT NULL PRIMARY KEY,
);

CREATE TABLE events (
    id      SERIAL NOT NULL PRIMARY KEY,
    name    TEXT NOT NULL,
);

CREATE TABLE subevents (
    id      SERIAL NOT NULL PRIMARY KEY,
    event   SERIAL NOT NULL REFERENCES events(id),
    comment TEXT NOT NULL,
);

CREATE TABLE joinevents (
    usrmail VARCHAR(255) NOT NULL REFERENCES users(mail),
    event   SERIAL NOT NULL REFERENCES events(id),
);

CREATE TABLE joinsubevents (
    usrmail VARCHAR(255) NOT NULL REFERENCES users(mail),
    subevt  SERIAL NOT NULL REFERENCES subevent(id),
    scanned BOOLEAN NOT NULL,
);

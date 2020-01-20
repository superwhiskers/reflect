//
//  reflect - a global mirror channel for discord guilds
//  Copyright (C) superwhiskers <whiskerdev@protonmail.com> 2020
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Affero General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU Affero General Public License for more details.
//
//  You should have received a copy of the GNU Affero General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

use log::LevelFilter;

use crate::types::DatabaseConfig;

#[inline(always)]
pub fn prefix() -> String {
    String::from("r~")
}

#[inline(always)]
pub fn log_file() -> String {
    String::from("output.log")
}

#[inline(always)]
pub fn log_level() -> LevelFilter {
    LevelFilter::Info
}

#[inline(always)]
pub fn database_configuration() -> DatabaseConfig {
    DatabaseConfig {
        host: database_host(),
        max_connections: database_max_connections(),
    }
}

#[inline(always)]
pub fn database_max_connections() -> u32 {
    20
}

#[inline(always)]
pub fn database_host() -> String {
    String::from("redis://localhost")
}

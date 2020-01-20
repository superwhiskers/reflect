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

/// a way to constantly retry an operation that returns a
/// Result until it succeeds. do not use unless you are sure
/// that a failiure once does not equate to a failiure
/// happening again
#[macro_export]
macro_rules! try_until_success {
    ($op:expr) => {
        loop {
            match $op {
                Ok(result) => break result,
                Err(_) => continue,
            }
        }
    };
}

/// succinctly express that you want to get the database handle
/// from the pool
#[macro_export]
macro_rules! get_db_handle {
    ($data:expr) => {
        match $data.get::<$crate::types::Database>() {
            Some(database) => $crate::try_until_success!(database.get()),
            None => panic!("the database wasn't initialized and placed into the data TypeMap (this is a severe bug)"),
        }
    };
}

/// simplify sending an error message in the command's channel
#[macro_export]
macro_rules! say_error {
    ($message:ident, $context:ident, $content:expr) => {
        $message
            .channel_id
            .say(&$context.http, format!("**Error:** {}", $content))?;
    };
}

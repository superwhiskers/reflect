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

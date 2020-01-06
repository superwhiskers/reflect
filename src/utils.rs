use serenity::{framework::standard::Args, model::channel::Message, prelude::*};

// TODO(superwhiskers): fully implement after adding usercache stuff
/// resolves a user from the arguments passed to a command
/// returns an error message if something fails
pub fn resolve_user(
    context: &mut Context,
    message: &Message,
    arguments: Args,
) -> Result<u64, &'static str> {
    macro_rules! usercache_lookup {
        ($context:ident, $message:ident, $arguments:ident) => {
            Err("Usercache support has not been implemented yet. Unable to retrieve user id from the provided argument")
        };
    }

    match arguments.is_empty() {
        true => Err("No user was provided"),
        false => match arguments.parse::<u64>() {
            Ok(parsed_user_id) => Ok(parsed_user_id),
            Err(_) => match arguments.parse::<String>() {
                Ok(parsed_user_mention) => match parsed_user_mention
                    .trim_end_matches('>')
                    .trim_start_matches("<@")
                    .trim_start_matches("!")
                    .parse::<u64>()
                {
                    Ok(parsed_user_id) => Ok(parsed_user_id),
                    Err(_) => usercache_lookup!(context, message, arguments),
                },
                Err(_) => usercache_lookup!(context, message, arguments),
            },
        },
    }
}

use log::{warn};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::prelude::*, prelude::*};

use crate::telephone::{create_chain_in_map, ChainStorage};

#[command]
#[aliases("make", "new")]
fn create(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single::<String>()?;
    {
        let mut data = ctx.data.write();
        let mut map = data
            .get_mut::<ChainStorage>()
            .expect("Expected ChainMap to exist in shared map");
        match create_chain_in_map(&mut map, &name, msg.author.id, msg.channel_id) {
            Ok(()) => (),
            Err(why) => {
                warn!("Error creating chain: {}", why);
            }
        };
    }
    Ok(())
}

use log::{warn};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::prelude::*, prelude::*};

use crate::telephone::{add_to_chain_map, ChainStorage};

#[command]
#[description = "Join a Telephone chain, if it is open"]
fn join(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single::<String>()?;
    {
        let mut data = ctx.data.write();
        let mut map = data
            .get_mut::<ChainStorage>()
            .expect("Expected ChaimMap to exist in shared storage!");
        match add_to_chain_map(&mut map, &name, msg.author.id) {
            Ok(()) => (),
            Err(why) => {
                warn!("Error for {} joining chain: {}", msg.author, why);
            }
        };
    }

    let outmsg = format!("Successfully joined chain \"{}\"!", name);
    let _ = msg.channel_id.say(&ctx.http, outmsg);

    Ok(())
}

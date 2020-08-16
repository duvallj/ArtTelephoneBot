use crate::telephone::{submit_to_chain_map, ChainStorage};
use log::{info, warn};
use serenity::framework::standard::{Args, macros::command, CommandResult};
use serenity::{model::prelude::*, prelude::*};

#[command]
#[aliases("upload")]
#[description = "Submits a picture to a telephone chain. Currently only supports raw attachments"]
fn submit(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single::<String>()?;
    if msg.attachments.len() == 0 {
        let _ = msg.channel_id.say(&ctx.http, "No attachments!");

        return Ok(());
    }

    // Only take the first attachment
    // TODO: Should probably say this somewhere
    let attachment = &msg.attachments[0];
    let content = match attachment.download() {
        Ok(res) => res,
        Err(why) => {
            warn!("Error downloading attachment: {:?}", why);
            let _ = msg
                .channel_id
                .say(&ctx.http, "Error downloading attachment!");

            return Ok(());
        }
    };

    info!("Downloading file {}", &attachment.filename);

    {
        let mut data = ctx.data.write();
        let mut map = data
            .get_mut::<ChainStorage>()
            .expect("Expected ChainMap to exist in shared map");
        match submit_to_chain_map(&mut map, &name, msg.author.id, content) {
            Ok(()) => (),
            Err(why) => {
                warn!("Error submiting data to chain: {}", why);
            }
        };
    }

    let _ = msg
        .channel_id
        .say(&ctx.http, "Successfully downloaded attachment!");

    Ok(())
}

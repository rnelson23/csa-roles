use std::error::Error;

use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), Box<dyn Error>> {
    let roles = command.guild_id.unwrap().roles(&ctx.http).await?;
    let (_, top_role) = roles.iter().find(|(_, r)| r.name == "CSA Roles").unwrap();
    let mut description = String::new();

    for (i, (role_id, role)) in roles.iter().enumerate() {
        if role.position < top_role.position && role_id.as_u64() != command.guild_id.unwrap().as_u64() {
            description.push_str(&format!("{}. <@&{}>\n", i + 1, role.id));
        }
    }

    let mut embed = CreateEmbed::default();
    embed.description(description);

    crate::reply_embed(ctx, command, embed).await?;
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list")
        .description("List available roles")
}

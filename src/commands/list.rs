use std::error::Error;

use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::model::guild::Role;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), Box<dyn Error>> {
    let roles = command.guild_id.unwrap().roles(&ctx.http).await?;
    let (_, top_role) = roles.iter().find(|(_, r)| r.name == "CSA Roles").unwrap();
    let mut addable_roles: Vec<&Role> = vec![];

    for (_, role) in roles.iter() {
        if role.position < top_role.position && role.id.as_u64() != command.guild_id.unwrap().as_u64() {
            addable_roles.push(role);
        }
    }

    addable_roles.sort_by(|a, b| b.position.cmp(&a.position));

    let mut description = String::new();
    let mut count = 1;

    for role in addable_roles {
        description.push_str(&format!("{}. <@&{}>\n", count, role.id));
        count += 1;
    }

    let mut embed = CreateEmbed::default();
    embed.description(description);
    embed.color(0x3aadc8);

    crate::reply_embed(ctx, command, embed).await?;
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list")
        .description("List available roles")
}

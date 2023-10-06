use std::error::Error;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), Box<dyn Error>> {
    let role = crate::get_role_option(command, 0).unwrap();
    let roles = command.guild_id.unwrap().roles(&ctx.http).await?;
    let (_, top_role) = roles.iter().find(|(_, r)| r.name == "CSA Roles").unwrap();

    let mut member = command.guild_id.unwrap().member(&ctx.http, command.user.id).await?;

    if !member.roles.contains(&role.id) {
        crate::reply_ephemeral(ctx, command, "You don't have that role").await?;
        return Ok(())
    }

    if roles.get(&role.id).unwrap().position > top_role.position {
        crate::reply_ephemeral(ctx, command, "You can't remove that role from yourself").await?;
        return Ok(())
    }

    member.remove_role(&ctx.http, role.id).await?;

    crate::reply(ctx, command, &format!("You have removed the <@&{}> role!", role.id)).await?;
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("remove")
        .description("Removes a role to you")
        .create_option(|option| {
            option
                .name("role")
                .description("The role to remove from you")
                .kind(CommandOptionType::Role)
                .required(true)
        })
}

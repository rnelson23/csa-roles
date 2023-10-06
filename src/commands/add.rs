use std::error::Error;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), Box<dyn Error>> {
    let role = crate::get_role_option(command, 0).unwrap();
    let roles = command.guild_id.unwrap().roles(&ctx.http).await?;
    let (_, top_role) = roles.iter().find(|(_, r)| r.name == "CSA Roles").unwrap();

    if roles.get(&role.id).unwrap().position > top_role.position {
        crate::reply(ctx, command, "You can't add that role to yourself").await?;
        return Ok(())
    }

    let mut member = command.guild_id.unwrap().member(&ctx.http, command.user.id).await?;
    member.add_role(&ctx.http, role.id).await?;

    crate::reply(ctx, command, &format!("You have been given the <@&{}> role!", role.id)).await?;
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add")
        .description("Adds a role to you")
        .create_option(|option| {
            option
                .name("role")
                .description("The role to add to you")
                .kind(CommandOptionType::Role)
                .required(true)
        })
}

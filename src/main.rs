mod commands;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use dotenv::dotenv;
use std::env;
use std::error::Error;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::Role;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer")
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::add::register(command))
                .create_application_command(|command| commands::list::register(command))
                .create_application_command(|command| commands::remove::register(command))
        }).await;

        println!("I now have the following guild slash commands: {:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Got command {:?}", command.data.name);

            if let Err(why) = match command.data.name.as_str() {
                "add" => commands::add::run(&ctx, &command).await,
                "list" => commands::list::run(&ctx, &command).await,
                "remove" => commands::remove::run(&ctx, &command).await,
                _ => Ok(())
            } {
                println!("Cannot execute slash command: {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

async fn reply(ctx: &Context, command: &ApplicationCommandInteraction, content: &str) -> Result<(), Box<dyn Error>> {
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content(content)
                })
        })
        .await
    {
        return Err(why.into());
    }

    Ok(())
}

async fn reply_embed(ctx: &Context, command: &ApplicationCommandInteraction, embed: CreateEmbed) -> Result<(), Box<dyn Error>> {
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.add_embed(embed)
                })
        })
        .await
    {
        return Err(why.into());
    }

    Ok(())
}

fn get_role_option(command: &ApplicationCommandInteraction, index: usize) -> Option<&Role> {
    match command.data.options.get(index) {
        Some(option) => match option.resolved.as_ref() {
            Some(CommandDataOptionValue::Role(role)) => Some(role),
            _ => None
        },
        None => None
    }
}

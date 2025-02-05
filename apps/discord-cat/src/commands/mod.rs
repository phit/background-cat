use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
    utils::Colour,
};

use log::{debug, error};
use serde::Deserialize;

mod xkcd;
use xkcd::XKCD_COMMAND;

macro_rules! static_text_command {
    ( $($name:ident $($($aliases:literal)+)?, $title:tt, $message:tt;)+ ) => {
        #[group("Text")]
        #[commands( $($name),* )]
        struct StaticText;

        $(
            #[command]
            $( #[aliases($($aliases),+)] )?
            async fn $name(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
                if let Err(why) = msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title($title);
                        e.colour(Colour::DARK_TEAL);
                        e.description($message);
                        e.footer(|f| {
                            f.icon_url("https://cdn.discordapp.com/emojis/280120125284417536.png?v=1")
                        })
                    });
                    debug!("Message: {:?}", m);
                    m
                }).await {
                    error!("couldn't send message: {}", why);
                }
                Ok(())
            }
        )+
    };
}

macro_rules! static_image_command {
    ( $($name:ident $($($aliases:literal)+)?, $image:tt$(, $message:tt)?;)+ ) => {
        #[group("Images")]
        #[commands( $($name,)* )]
        struct StaticImage;

        $(
            #[command]
            $( #[aliases($($aliases),+)] )?
            async fn $name(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
                if let Err(why) = msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.image($image);
                        $(e.title($message);)?
                        e.colour(Colour::DARK_TEAL);
                        e.footer(|f| {
                            f.icon_url("https://cdn.discordapp.com/emojis/280120125284417536.png?v=1")
                        })
                    });
                    debug!("Message: {:?}", m);
                    m
                }).await {
                    error!("couldn't send message: {}", why);
                }
                Ok(())
            }
        )+
    };

}

// Format: Name (Optional Alias1 Alias2...) , Title , Message ;
static_text_command! {
    install_java "ijava", "Please install the right Java version:",
        "https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java";
    too_much_ram "tmram" "vazkiiram",
        "Allocating too much RAM to Minecraft is bad for performance:",
        "https://vazkii.net/blog_archive/#blog/ram-explanation";
    mod_repost "repost" "vazkiirepost" "9mc" "9minecraft",
        "Please make sure you only download mods from reputable sources.",
        "For more info, please read https://vazkii.net/repost/";
    ipv4,
        "Add this to your Java arguments to make Minecraft prefer IPv4 over IPv6:",
        "`-Djava.net.preferIPv4Stack=true`";
    optifine,
        "To use OptiFine with MultiMC, please read this page:",
        "https://github.com/MultiMC/MultiMC5/wiki/MultiMC-and-OptiFine";
}

// Format: Name (Optional Alias1 Alias2...) , Image Link (, Optional Message) ;
static_image_command! {
    upload_log "log", "https://cdn.discordapp.com/attachments/531598137790562305/575381000398569493/unknown.png",
        "Please upload your log:";
    select_java "sjava", "https://cdn.discordapp.com/attachments/531598137790562305/575378380573114378/unknown.png",
        "Please select your Java version in the MultiMC settings:";
    select_memory "smemory" "sram", "https://cdn.discordapp.com/attachments/531598137790562305/575376840173027330/unknown.png",
        "Please set your instance memory allocation:";
    install_forge "iforge", "https://cdn.discordapp.com/attachments/531598137790562305/575385471207866388/Install_Forge_in_MultiMC.gif",
        "How to install Forge:";
    javaarg "javaargs" "jarg" "jargs",
        "https://cdn.discordapp.com/attachments/362205883218001920/711410345301770300/MultiMC_JVM_Args.png";
    multimc_dev "dev", "https://cdn.discordapp.com/attachments/134843027553255425/855880510031003728/unknown.png",
        "How to switch to the MultiMC development version:";
}

#[group]
#[commands(info)]
struct Other;

#[command]
async fn info(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let creator_name = match UserId::from(185_461_862_878_543_872).to_user(ctx).await {
        Ok(o) => o.tag(),
        Err(why) => {
            error!("Couldn't get info about creator: {}", why);
            "<Error getting name>".to_string()
        }
    };
    if let Err(why) = msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("<:backgroundcat:280120125284417536>A bot to parse logfiles on the MultiMC discord<:backgroundcat:280120125284417536>");
                    e.colour(Colour::DARK_TEAL);
                    e.description(format!(r"
Developed by {}.
To start, just upload a log from MultiMC. (Type `-log` for help)

[Source Code available under AGPLv3](https://github.com/MultiMC/background-cat)
", creator_name))
                });
                m
            }).await {
                error!("Couldn't send info message: {}", why)
            }
    Ok(())
}

#[group]
#[commands(drama, xkcd)]
struct Fun;

#[derive(Deserialize)]
struct Drama {
    drama: String,
    version: String,
    seed: String,
}

#[command]
#[description = "Generate some Minecraft modding drama."]
#[description = "Add 'fabric' as the first argument for Fabric-brand drama"]
#[usage = "[fabric]"]
async fn drama(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    const MC_DRAMA: &str = "https://ftb-drama.herokuapp.com";
    const FABRIC_DRAMA: &str = "https://fabric-drama.herokuapp.com";

    let dest = if msg.content.to_lowercase().contains("fabric") {
        FABRIC_DRAMA
    } else {
        MC_DRAMA
    }
    .to_owned();

    let drama = reqwest::get(&(dest.clone() + "/json"))
        .await?
        .json::<Drama>()
        .await?;
    let permalink = dest + "/" + &drama.version + "/" + &drama.seed;

    if let Err(why) = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("MC Drama Generator");
                e.description(&drama.drama);
                e.colour(Colour::DARK_TEAL);
                e.footer(|f| {
                    f.icon_url("https://cdn.discordapp.com/emojis/280120125284417536.png?v=1");
                    f.text(permalink)
                })
            })
        })
        .await
    {
        error!("Couldn't send drama: {}", why);
    }

    Ok(())
}

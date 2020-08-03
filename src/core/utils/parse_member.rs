use futures::{stream, StreamExt};

use regex::Regex;
use serenity::{
    client::Context,
    model::{channel::Message, guild::Member, id::UserId},
};

pub async fn parse_member(ctx: &Context, msg: Message, user: String) -> Result<Member, String> {
    let mut members = vec![];

    if let Ok(id) = user.parse::<u64>() {
        let member = &msg.guild_id.unwrap().member(ctx, id).await;
        match member {
            Ok(m) => Ok(m.to_owned()),
            Err(why) => Err(why.to_string()),
        }
    } else if user.starts_with("<@") && user.ends_with('>') {
        let re = Regex::new("[<@!>]").unwrap();
        let member_id = re.replace_all(&user, "").into_owned();
        let member = &msg
            .guild_id
            .unwrap()
            .member(ctx, UserId(member_id.parse::<u64>().unwrap()))
            .await;

        match member {
            Ok(m) => Ok(m.to_owned()),
            Err(why) => Err(why.to_string()),
        }
    } else {
        let guild = &msg.guild(ctx).await.unwrap();
        let user = user.split('#').next().unwrap();

        for m in guild.members.values() {
            if m.display_name() == std::borrow::Cow::Borrowed(user) || m.user.name == user {
                members.push(m);
            }
        }

        if members.is_empty() {
            let similar_members = &guild.members_containing(&user, false, false).await;

            let mut members_string = stream::iter(similar_members.iter())
                .map(|m| async move {
                    let member = &m.0.user;
                    format!("`{}`|", member.name)
                })
                .fold(String::new(), |mut acc, c| async move {
                    acc.push_str(&c.await);
                    acc
                })
                .await;

            let message = {
                if members_string == "" {
                    format!("No member named '{}' was found.", user.replace("@", ""))
                } else {
                    members_string.pop();
                    format!(
                        "No member named '{}' was found.\nDid you mean: {}",
                        user.replace("@", ""),
                        members_string.replace("@", "")
                    )
                }
            };
            Err(message)
        } else if members.len() == 1 {
            Ok(members[0].to_owned())
        } else {
            let mut members_string = stream::iter(members.iter())
                .map(|m| async move {
                    let member = &m.user;
                    format!("`{}#{}`|", member.name, member.discriminator)
                })
                .fold(String::new(), |mut acc, c| async move {
                    acc.push_str(&c.await);
                    acc
                })
                .await;

            members_string.pop();

            let message = format!(
                "Multiple members with the same name where found: '{}'",
                &members_string
            );
            Err(message)
        }
    }
}

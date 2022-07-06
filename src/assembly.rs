/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use multimap::MultiMap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::discord_api;
use crate::model;

pub(crate) fn assemble_roles(
    members: &[discord_api::GuildMember],
    roles: &[discord_api::Role],
    roles_excluded: &HashSet<String>,
) -> Vec<model::Role> {
    let user_ids_to_names = map_user_ids_to_names(members);
    let role_ids_to_names = map_role_ids_to_names(roles);
    let role_ids_to_colors = map_role_ids_to_colors(roles);
    let role_ids_to_user_ids = map_role_ids_to_user_ids(members);

    let mut roles: Vec<model::Role> = vec![];

    for (role_id, user_ids) in role_ids_to_user_ids.iter_all() {
        let role_name = role_ids_to_names.get(role_id).unwrap();
        if roles_excluded.contains(*role_name) {
            continue;
        }

        let mut member_names = get_member_names(user_ids, &user_ids_to_names);
        member_names.sort_unstable_by_key(|name| name.to_lowercase());

        let color_hex = role_ids_to_colors.get(role_id).unwrap();

        roles.push(model::Role {
            name: role_name.to_string(),
            member_names,
            color_hex: color_hex.to_string(),
        });
    }

    roles.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    roles
}

fn map_user_ids_to_names(members: &[discord_api::GuildMember]) -> HashMap<&String, &String> {
    let users = members.iter().map(|m| &m.user);
    let mut user_ids_to_names = HashMap::with_capacity(users.len());
    for user in users {
        user_ids_to_names.insert(&user.id, &user.username);
    }
    user_ids_to_names
}

fn map_role_ids_to_names(roles: &[discord_api::Role]) -> HashMap<&String, &String> {
    let mut role_ids_to_names = HashMap::with_capacity(roles.len());
    for role in roles {
        role_ids_to_names.insert(&role.id, &role.name);
    }
    role_ids_to_names
}

fn map_role_ids_to_colors(roles: &[discord_api::Role]) -> HashMap<&String, String> {
    let mut role_ids_to_colors = HashMap::with_capacity(roles.len());
    for role in roles {
        let color_hex = convert_discord_color_to_hex(role.color);
        role_ids_to_colors.insert(&role.id, color_hex);
    }
    role_ids_to_colors
}

fn map_role_ids_to_user_ids(members: &[discord_api::GuildMember]) -> MultiMap<&String, &String> {
    let mut role_ids_to_user_ids = MultiMap::new();
    for member in members {
        for role_id in &member.role_ids {
            role_ids_to_user_ids.insert(role_id, &member.user.id);
        }
    }
    role_ids_to_user_ids
}

fn get_member_names(
    user_ids: &[&String],
    user_ids_to_names: &HashMap<&String, &String>,
) -> Vec<String> {
    let mut member_names: Vec<String> = user_ids
        .iter()
        .map(|id| user_ids_to_names.get(id).unwrap().to_string())
        .collect();
    member_names.sort_unstable_by_key(|name| name.to_lowercase());
    member_names
}

fn convert_discord_color_to_hex(discord_color: u32) -> String {
    let red = discord_color >> 16 & 0xFF;
    let green = discord_color >> 8 & 0xFF;
    let blue = discord_color & 0xFF;

    format!("#{:0>2x}{:0>2x}{:0>2x}", red, green, blue)
}

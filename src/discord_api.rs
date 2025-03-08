/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use serde::Deserialize;
use ureq::{Agent, AgentBuilder, Response};

#[derive(Debug, Deserialize)]
pub(crate) struct GuildMember {
    pub user: User,

    #[serde(rename = "roles")]
    pub role_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct User {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Role {
    pub id: String,
    pub name: String,
    pub color: u32,
}

/// Client for Discord API
pub(crate) struct Client {
    bot_token: String,
    agent: Agent,
}

impl Client {
    pub(crate) fn new(bot_token: &str) -> Self {
        Self {
            bot_token: bot_token.to_string(),
            agent: AgentBuilder::new().build(),
        }
    }

    /// Fetch members of a guild.
    pub(crate) fn get_guild_members(&self, guild_id: &str) -> Result<Vec<GuildMember>> {
        let url = format!("https://discord.com/api/v9/guilds/{guild_id}/members?limit=1000");
        let response = self.query(&url)?;
        Ok(response.into_json()?)
    }

    /// Fetch roles for a guild.
    pub(crate) fn get_guild_roles(&self, guild_id: &str) -> Result<Vec<Role>> {
        let url = format!("https://discord.com/api/v9/guilds/{guild_id}/roles");
        let response = self.query(&url)?;
        Ok(response.into_json()?)
    }

    /// Query the Discord API.
    fn query(&self, url: &str) -> Result<Response> {
        let bot_token = &self.bot_token;
        let authz_value = format!("Bot {bot_token}");
        let request = self.agent.get(url).set("Authorization", &authz_value);
        Ok(request.call()?)
    }
}

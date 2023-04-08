//! Twitch API helpers.

use anyhow::Result;
use bytes::Bytes;
use futures_core::Stream;
use reqwest::{header, Client, Method, Url};
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::api::RequestBuilder;
use crate::oauth2;

pub(crate) const CLIPS_URL: &str = "http://clips.twitch.tv";
const API_TWITCH_URL: &str = "https://api.twitch.tv";
const GQL_URL: &str = "https://gql.twitch.tv/gql";

const GQL_CLIENT_ID: &str = "kimne78kx3ncx6brgo4mv6wki5h1ko";
/// Common header.
const BROADCASTER_ID: &str = "broadcaster_id";

mod gql;
mod model;
pub(crate) mod pubsub;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("missing user")]
    MissingUser,
}

pub(crate) use self::model::*;

/// Twitch API client.
#[derive(Clone, Debug)]
pub(crate) struct Twitch {
    client: Client,
    client_id_header: header::HeaderName,
    api_url: Url,
    gql_url: Url,
    pub(crate) token: oauth2::SyncToken,
}

impl Twitch {
    /// Create a new Twitch API client.
    pub(crate) fn new(token: oauth2::SyncToken) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            client_id_header: str::parse::<header::HeaderName>("Client-ID")?,
            api_url: str::parse::<Url>(API_TWITCH_URL)?,
            gql_url: str::parse::<Url>(GQL_URL)?,
            token,
        })
    }

    /// Get display badges through GQL.
    pub(crate) async fn gql_display_badges(
        &self,
        login: &str,
        channel: &str,
    ) -> Result<Option<gql::badges::ResponseData>> {
        use graphql_client::{GraphQLQuery as _, Response};

        let body = gql::Badges::build_query(gql::badges::Variables {
            login: login.to_string(),
            channel_login: channel.to_string(),
        });

        let res = self
            .gql()
            .body(serde_json::to_vec(&body)?)
            .execute()
            .await?
            .json::<Response<gql::badges::ResponseData>>()?
            .data;

        Ok(res)
    }

    /// Get chatters for the given broadcaster.
    pub(crate) fn chatters(
        &self,
        moderator_id: &str,
        broadcaster_id: &str,
    ) -> impl Stream<Item = Result<Chatter>> + '_ {
        let mut req = self.new_api(Method::GET, &["chat", "chatters"]);

        req.query_param("moderator_id", moderator_id)
            .query_param("broadcaster_id", broadcaster_id);

        page(req)
    }

    /// Get moderators for the current broadcaster.
    pub(crate) fn moderators(
        &self,
        broadcaster_id: &str,
    ) -> impl Stream<Item = Result<Chatter>> + '_ {
        let mut req = self.new_api(Method::GET, &["moderation", "moderators"]);
        req.query_param("broadcaster_id", broadcaster_id);
        page(req)
    }

    /// Get VIPs for the current broadcaster.
    pub(crate) fn vips(&self, broadcaster_id: &str) -> impl Stream<Item = Result<Chatter>> + '_ {
        let mut req = self.new_api(Method::GET, &["channels", "vips"]);
        req.query_param("broadcaster_id", broadcaster_id);
        page(req)
    }

    /// Search for a category with the given name.
    pub(crate) fn categories<'a>(
        &'a self,
        query: &str,
    ) -> impl Stream<Item = Result<new::Category>> + 'a {
        let mut req = self.new_api(Method::GET, &["search", "categories"]);
        req.query_param("query", query);
        page(req)
    }

    /// Get information on a user.
    pub(crate) fn subscriptions<'a>(
        &'a self,
        broadcaster_id: &str,
        user_ids: Vec<String>,
    ) -> impl Stream<Item = Result<new::Subscription>> + 'a {
        let mut req = self.new_api(Method::GET, &["subscriptions"]);
        req.query_param(BROADCASTER_ID, broadcaster_id);

        for user_id in &user_ids {
            req.query_param("user_id", user_id);
        }

        page(req)
    }

    /// Create a clip for the given broadcaster.
    pub(crate) async fn create_clip(&self, broadcaster_id: &str) -> Result<Option<new::Clip>> {
        let res = self
            .new_api(Method::POST, &["clips"])
            .query_param(BROADCASTER_ID, broadcaster_id)
            .execute()
            .await?
            .json::<Data<Vec<new::Clip>>>()?;

        Ok(res.data.into_iter().next())
    }

    /// Get stream information.
    pub(crate) async fn streams(
        &self,
        user_id: &str,
    ) -> impl Stream<Item = Result<new::Stream>> + '_ {
        let mut req = self.new_api(Method::GET, &["streams"]);
        req.query_param("user_id", user_id);
        page(req)
    }

    /// Update the status of a redemption.
    pub(crate) async fn patch_redemptions(
        &self,
        broadcaster_id: &str,
        redemption: &pubsub::Redemption,
        status: pubsub::Status,
    ) -> Result<()> {
        use serde::Serialize;

        let mut req = self.new_api(
            Method::PATCH,
            &["channel_points", "custom_rewards", "redemptions"],
        );

        req.header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json")
            .query_param(BROADCASTER_ID, broadcaster_id)
            .query_param("id", &redemption.id)
            .query_param("reward_id", &redemption.reward.id)
            .body(serde_json::to_vec(&UpdateRedemption { status })?);

        req.execute()
            .await?
            .json::<Data<Vec<serde_json::Value>>>()?;

        return Ok(());

        #[derive(Serialize)]
        struct UpdateRedemption {
            status: pubsub::Status,
        }
    }

    /// Get the channel associated with the current authentication.
    pub(crate) async fn user(&self) -> Result<new::User> {
        let req = self.new_api(Method::GET, &["users"]);
        let data = req.execute().await?.json::<Data<Vec<new::User>>>()?;
        let user = data.data.into_iter().next().ok_or(Error::MissingUser)?;
        Ok(user)
    }

    /// Get the channel associated with the specified broadcaster id.
    pub(crate) async fn channels(&self, broadcaster_id: &str) -> Result<Option<new::Channel>> {
        let mut req = self.new_api(Method::GET, &["channels"]);
        req.query_param(BROADCASTER_ID, broadcaster_id);
        let result = req.execute().await?.json::<Data<Vec<new::Channel>>>()?;
        Ok(result.data.into_iter().next())
    }

    /// Get emotes by sets.
    pub(crate) async fn emote_set(&self, id: &str) -> Result<Vec<new::Emote>> {
        let mut req = self.new_api(Method::GET, &["chat", "emotes", "set"]);
        req.query_param("emote_set_id", id);
        Ok(req.execute().await?.json::<Data<Vec<new::Emote>>>()?.data)
    }

    /// Update the channel information.
    pub(crate) async fn patch_channel(
        &self,
        broadcaster_id: &str,
        request: new::ModifyChannelRequest<'_>,
    ) -> Result<()> {
        let body = Bytes::from(serde_json::to_vec(&request)?);

        self.new_api(Method::PATCH, &["channels"])
            .query_param(BROADCASTER_ID, broadcaster_id)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .execute()
            .await?
            .ok()
    }

    /// Get request against API.
    fn new_api<'a>(&'a self, method: Method, path: &[&str]) -> RequestBuilder<'a> {
        let mut url = self.api_url.clone();

        {
            let mut url_path = url.path_segments_mut().expect("bad base");
            url_path.push("helix");
            url_path.extend(path);
        }

        let mut req = RequestBuilder::new(&self.client, method, url);
        req.token(&self.token)
            .client_id_header(&self.client_id_header);
        req
    }

    /// Access GQL client.
    fn gql(&self) -> RequestBuilder<'_> {
        let mut req = RequestBuilder::new(&self.client, Method::POST, self.gql_url.clone());

        req.header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json")
            .header(self.client_id_header.clone(), GQL_CLIENT_ID);

        req
    }
}

/// Perform pagination over the given request.
fn page<'a, T: 'a>(request: RequestBuilder<'a>) -> impl Stream<Item = Result<T>> + 'a
where
    T: DeserializeOwned,
{
    async_stream::try_stream! {
        let initial = request.execute().await?.json::<new::Page<T>>()?;
        let mut page = initial.data.into_iter();
        let mut pagination = initial.pagination;

        loop {
            for item in page.by_ref() {
                yield item;
            }

            let cursor = match pagination.as_ref().and_then(|p| p.cursor.as_ref()) {
                Some(cursor) => cursor,
                None => break,
            };

            let mut next = request.clone();
            next.query_param("after", cursor);
            let next = next.execute().await?.json::<new::Page<T>>()?;
            page = next.data.into_iter();
            pagination = next.pagination;
        }
    }
}

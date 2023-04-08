mod base;
pub(crate) mod bttv;
pub(crate) mod ffz;
pub(crate) mod github;
pub(crate) mod nightbot;
pub(crate) mod open_weather_map;
mod provider;
pub(crate) mod setbac;
pub(crate) mod speedrun;
pub(crate) mod spotify;
pub(crate) mod tduva;
pub(crate) mod twitch;
pub(crate) mod youtube;

pub(crate) use self::base::RequestBuilder;
pub(crate) use self::bttv::BetterTTV;
pub(crate) use self::ffz::FrankerFaceZ;
pub(crate) use self::github::GitHub;
pub(crate) use self::nightbot::NightBot;
pub(crate) use self::open_weather_map::OpenWeatherMap;
pub(crate) use self::provider::{twitch_clients_task, TwitchAndUser, User};
pub(crate) use self::setbac::Setbac;
pub(crate) use self::speedrun::Speedrun;
pub(crate) use self::spotify::Spotify;
pub(crate) use self::tduva::Tduva;
pub(crate) use self::twitch::Twitch;
pub(crate) use self::youtube::YouTube;

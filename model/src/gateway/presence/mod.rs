mod activity;
mod activity_assets;
mod activity_flags;
mod activity_party;
mod activity_secrets;
mod activity_timestamps;
mod activity_type;
mod status;

pub use self::{
    activity::Activity,
    activity_assets::ActivityAssets,
    activity_flags::ActivityFlags,
    activity_party::ActivityParty,
    activity_secrets::ActivitySecrets,
    activity_timestamps::ActivityTimestamps,
    activity_type::ActivityType,
    status::Status,
};

use crate::{
    id::UserId,
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Presence {
    pub activity: Option<Activity>,
    pub last_modified: Option<u64>,
    pub nick: Option<String>,
    pub status: Status,
    pub user: UserOrId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UserOrId {
    User(User),
    UserId(UserId),
}
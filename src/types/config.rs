use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct Configuration {
    /// app info
    pub(crate) app_info: AppInfo,
    pub(crate) default: Default,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AppInfo {
    pub(crate) app_id: String,
    pub(crate) app_secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Default {
    pub(crate) from: String,
    pub(crate) to: String,
}
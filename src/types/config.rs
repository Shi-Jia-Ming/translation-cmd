use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Configuration {
    /// app info
    pub(crate) app_info: AppInfo,
    pub(crate) default: Default,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AppInfo {
    pub(crate) api_version: String,
    pub(crate) app_id: String,
    pub(crate) app_secret: String,
    pub(crate) momo_token: String,
    pub(crate) deeplx_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Default {
    pub(crate) from: String,
    pub(crate) to: String,
}

pub(crate) enum ApiVersion {
    BAIDU,
    MOMO,
    DEEPLX,
    ILLEGAL
}

impl Configuration {
    pub(crate) fn to_str(self) -> String {
        format!("Configuration list(use variable name):\n{}{}", self.app_info.to_str(), self.default.to_str())
    }

    pub(crate) fn load_app_info(&self) -> (String, String) {
        (self.app_info.app_id.clone(), self.app_info.app_secret.clone())
    }

    pub(crate) fn load_deeplx_token(&self) -> String {
        self.app_info.deeplx_token.clone()
    }

    pub(crate) fn get_api_version(&self) -> ApiVersion {
        match self.app_info.api_version.as_str() {
            "baidu" => ApiVersion::BAIDU,
            "momo" => ApiVersion::MOMO,
            "deeplx" => ApiVersion::DEEPLX,
            _ => ApiVersion::ILLEGAL,
        }
    }

    pub(crate) fn load_default_option(&self) -> (String, String) {
        (self.default.from.clone(), self.default.to.clone())
    }
}

impl AppInfo {
    fn to_str(self) -> String {
        format!("\napp_info\n  - api_version: {}\n  - app_id: {}\n  - app_secret: {}\n  - momo_token: {}\n  - deeplx_token: {}\n",
                self.api_version, self.app_id, self.app_secret, self.momo_token, self.deeplx_token)
    }
}

impl Default {
    fn to_str(self) -> String {
        format!("\ndefault\n  - from: {}\n  - to: {}\n", self.from, self.to)
    }
}
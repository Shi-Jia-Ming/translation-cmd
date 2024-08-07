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

impl Configuration {
    pub(crate) fn to_str(self) -> String {
        let mut result: String = String::new();
        result += "Configuration list(use variable name):\n";
        result += &*self.app_info.to_str();
        result += &*self.default.to_str();

        result.clone()
    }
}

impl AppInfo {
    fn to_str(self) -> String {
        let mut result: String = String::new();
        result += "\n[app_info]\n";
        result += &*format!("  - app_id: {}\n", self.app_id).to_string();
        result += &*format!("  - app_secret: {}\n\n", self.app_secret).to_string();

        result.clone()
    }
}

impl Default {
    fn to_str(self) -> String {
        let mut result: String = String::new();
        result += "\n[app_info]\n";
        result += &*format!("  - from: {}\n", self.from).to_string();
        result += &*format!("  - to: {}\n", self.to).to_string();

        result.clone()
    }
}
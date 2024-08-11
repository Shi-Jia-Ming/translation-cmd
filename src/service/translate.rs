use crypto::{digest::Digest, md5::Md5};
use rand::Rng;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

use crate::service::config::ConfigHandler;
use crate::types::args::TransArgs;

#[derive(Debug, Deserialize, Serialize)]
struct TransResult {
    src: String,
    dst: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    from: String,
    to: String,
    trans_result: Vec<TransResult>,
}

pub(super) struct TranslateHandler;

impl TranslateHandler {
    /// generate the url for common translate service.
    fn generate_common_translate_url(from: &str, to: &str, target: &str, app_id: &str, app_secret: &str) -> String {
        let salt: String = rand::thread_rng().gen_range(1..=100).to_string();

        // generate sign code
        let mut hasher: Md5 = Md5::new();
        let data: String = format!("{}{}{}{}", app_id, target, salt.to_string(), app_secret);
        hasher.input_str(&data);
        let sign: String = hasher.result_str();

        // generate url
        format!("http://api.fanyi.baidu.com/api/trans/vip/translate?q={}&from={}&to={}&appid={}&salt={}&sign={}", encode(&target), from, to, app_id, salt, sign)
    }

    /// call the translation service.
    pub fn common_translate(url: &str) -> String {
        let client: Client = Client::new();

        let res: String = client.get(url)
            .send().expect("Failed to send request")
            .text().expect("Failed to read response");

        let response: Response = serde_json::from_str(&res).unwrap();

        response.trans_result[0b0].dst.clone()
    }
}

pub(crate) fn translate(args: TransArgs) -> String {
    // load configuration
    let (config, _) = ConfigHandler::load_config();

    let (default_from, default_to) = config.load_default_option();
    let (app_id, app_secret) = config.load_app_info();

    let from = if args.from == "default" { default_from } else { args.from };
    let to = if args.to == "default" { default_to } else { args.to };

    let url: String = TranslateHandler::generate_common_translate_url(&from, &to, &args.target, &app_id, &app_secret);

    TranslateHandler::common_translate(&url)
}
use crypto::{digest::Digest, md5::Md5};
use rand::Rng;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use urlencoding::encode;

use crate::service::config::ConfigHandler;
use crate::types::args::TransArgs;
use crate::types::config::ApiVersion;
use crate::utils::string::StringUtils;

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
    /// generate the url for common translate service in baidu.
    fn generate_common_translate_url_baidu(from: &str, to: &str, target: &str, app_id: &str, app_secret: &str) -> String {
        let salt: String = rand::thread_rng().gen_range(1..=100).to_string();

        // generate sign code
        let mut hasher: Md5 = Md5::new();
        let data: String = format!("{}{}{}{}", app_id, target, salt.to_string(), app_secret);
        hasher.input_str(&data);
        let sign: String = hasher.result_str();

        // generate url
        format!("http://api.fanyi.baidu.com/api/trans/vip/translate?q={}&from={}&to={}&appid={}&salt={}&sign={}", encode(&target), from, to, app_id, salt, sign)
    }

    /// generate the url for common translate service in deeplx.
    fn generate_common_translate_url_deeplx(source_lang: &str, target_lang: &str, target: &str, deeplx_token: &str) -> (String, HeaderMap, serde_json::value::Value) {
        let payload = json!({
            "text": target,
            "source_lang": source_lang,
            "target_lang": target_lang
        });

        let headers = reqwest::header::HeaderMap::from_iter([
            (CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap()),
            (AUTHORIZATION, HeaderValue::from_str(deeplx_token).unwrap()),
        ]);

        let url: String = "http://47.121.201.169:39770/translate".to_string();

        (url, headers, payload)
    }

    /// call the baidu translation service.
    pub fn common_translate_baidu(url: &str) -> String {
        let client: Client = Client::new();

        let res: String = client.get(url)
            .send().expect("Failed to send request")
            .text().expect("Failed to read response");

        let response: Response = serde_json::from_str(&res).unwrap();

        response.trans_result[0b0].dst.clone()
    }

    /// call the deeplx translation service.
    pub fn common_translate_deeplx(url: &str, headers: HeaderMap, payload: serde_json::value::Value) -> String {
        let client: Client = Client::new();

        let res: String = client.post(url)
            .headers(headers)
            .json(&payload)
            .send().expect("Failed to send request")
            .text().expect("Failed to read request");

        let json_res: serde_json::Value = serde_json::from_str(&res).unwrap();

        StringUtils::remove_quotes(&json_res["data"].to_string())
    }
}

pub(crate) fn translate(args: TransArgs) -> String {
    // load configuration
    let (config, _) = ConfigHandler::load_config();

    match config.get_api_version() {
        ApiVersion::BAIDU => {
            let (default_from, default_to) = config.load_default_option();
            let (app_id, app_secret) = config.load_app_info();

            let from = if args.from == "default" { default_from } else { args.from };
            let to = if args.to == "default" { default_to } else { args.to };

            let url: String = TranslateHandler::generate_common_translate_url_baidu(&from, &to, &args.target, &app_id, &app_secret);

            TranslateHandler::common_translate_baidu(&url)
        },
        ApiVersion::MOMO => { "Under development".parse().unwrap() },
        ApiVersion::DEEPLX => {
            let (default_from, default_to) = config.load_default_option();
            let deeplx_token = config.load_deeplx_token();

            let source_lang = if args.from == "default" { default_from } else { args.from };
            let target_lang = if args.to == "default" { default_to } else { args.to };

            let (url, headers, payload) = TranslateHandler::generate_common_translate_url_deeplx(&source_lang, &target_lang, &args.target, &deeplx_token);

            TranslateHandler::common_translate_deeplx(&url, headers, payload)
        },
        ApiVersion::ILLEGAL => { "Under development".parse().unwrap() },
    }
}

use crypto::{md5::Md5, digest::Digest};
use rand::Rng;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

#[derive(Debug, Deserialize, Serialize)]
struct TransResult {
    src: String,
    dst: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    from: String,
    to: String,
    trans_result: Vec<TransResult>
}

pub(crate) fn translate(from: &str, to: &str, target: &str, app_id: &str, app_secret: &str, default_from: &str, default_to: &str) -> String {
    let from = if from == "default" {default_from} else {from};
    let to = if to == "default" {default_to} else {to};
    let client: Client = Client::new();

    let salt: String = rand::thread_rng().gen_range(1..=100).to_string();

    // generate sign code
    let mut hasher: Md5 = Md5::new();
    let data: String = format!("{}{}{}{}", app_id, target, salt.to_string(), app_secret);
    hasher.input_str(&data);
    let sign: String = hasher.result_str();

    // generate url
    let url: String = format!("http://api.fanyi.baidu.com/api/trans/vip/translate?q={}&from={}&to={}&appid={}&salt={}&sign={}",
    encode(&target), from, to, app_id, salt, sign);
    
    let res: String = client.get(&url)
    .send().expect("Failed to send request")
    .text().expect("Failed to read response");
    
    let response: Response = serde_json::from_str(&res).unwrap();
    response.trans_result[0b0].dst.clone()
}
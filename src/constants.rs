use lazy_static::lazy_static;
use regex::Regex;
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde_json::{json, Value};

/*
* +------+
* | DATA |
* +------+
*/

pub fn subscriptions_mutation() -> Value {
    json!({
        "subscriptions":[
            {"subscriptionName":"messageAdded","query":null,"queryHash":"993dcce616ce18788af3cce85e31437abf8fd64b14a3daaf3ae2f0e02d35aa03"},
            {"subscriptionName":"messageCancelled","query":null,"queryHash":"14647e90e5960ec81fa83ae53d270462c3743199fbb6c4f26f40f4c83116d2ff"},
            {"subscriptionName":"messageDeleted","query":null,"queryHash":"91f1ea046d2f3e21dabb3131898ec3c597cb879aa270ad780e8fdd687cde02a3"},
            {"subscriptionName":"messageRead","query":null,"queryHash":"8c80ca00f63ad411ba7de0f1fa064490ed5f438d4a0e60fd9caa080b11af9495"},
            {"subscriptionName":"messageCreated","query":null,"queryHash":"47ee9830e0383f002451144765226c9be750d6c2135e648bced2ca7efc9d8a67"},
            {"subscriptionName":"messageStateUpdated","query":null,"queryHash":"117a49c685b4343e7e50b097b10a13b9555fedd61d3bf4030c450dccbeef5676"},
            {"subscriptionName":"messageAttachmentAdded","query":null,"queryHash":"65798bb2f409d9457fc84698479f3f04186d47558c3d7e75b3223b6799b6788d"},
            {"subscriptionName":"messageFollowupActionAdded","query":null,"queryHash":"d2e770beae7c217c77db4918ed93e848ae77df668603bc84146c161db149a2c7"},
            {"subscriptionName":"messageMetadataUpdated","query":null,"queryHash":"71c247d997d73fb0911089c1a77d5d8b8503289bc3701f9fb93c9b13df95aaa6"},
            {"subscriptionName":"messageTextUpdated","query":null,"queryHash":"800eea48edc9c3a81aece34f5f1ff40dc8daa71dead9aec28f2b55523fe61231"},
            {"subscriptionName":"jobStarted","query":null,"queryHash":"17099b40b42eb9f7e32323aa6badc9283b75a467bc8bc40ff5069c37d91856f6"},
            {"subscriptionName":"jobUpdated","query":null,"queryHash":"e8e492bfaf5041985055d07ad679e46b9a6440ab89424711da8818ae01d1a1f1"},
            {"subscriptionName":"viewerStateUpdated","query":null,"queryHash":"3b2014dba11e57e99faa68b6b6c4956f3e982556f0cf832d728534f4319b92c7"},
            {"subscriptionName":"unreadChatsUpdated","query":null,"queryHash":"5b4853e53ff735ae87413a9de0bce15b3c9ba19102bf03ff6ae63ff1f0f8f1cd"},
            {"subscriptionName":"chatTitleUpdated","query":null,"queryHash":"ee062b1f269ecd02ea4c2a3f1e4b2f222f7574c43634a2da4ebeb616d8647e06"},
            {"subscriptionName":"knowledgeSourceUpdated","query":null,"queryHash":"7de63f89277bcf54f2323008850573809595dcef687f26a78561910cfd4f6c37"},
            {"subscriptionName":"messagePointLimitUpdated","query":null,"queryHash":"ed3857668953d6e8849c1562f3039df16c12ffddaaac1db930b91108775ee16d"},
            {"subscriptionName":"chatMemberAdded","query":null,"queryHash":"21ef45e20cc8120c31a320c3104efe659eadf37d49249802eff7b15d883b917b"},
            {"subscriptionName":"chatSettingsUpdated","query":null,"queryHash":"3b370c05478959224e3dbf9112d1e0490c22e17ffb4befd9276fc62e196b0f5b"},
            {"subscriptionName":"chatModalStateChanged","query":null,"queryHash":"f641bc122ac6a31d466c92f6c724343688c2f679963b7769cb07ec346096bfe7"}]
    })
}

/*
* +---------+
* | BUNDLES |
* +---------+
*/

lazy_static! {
    pub static ref FORM_KEY_PATTERN: Regex =
        Regex::new(r#"window\.([a-zA-Z0-9]+)=function\(\)\{return window"#).unwrap();
    pub static ref WINDOW_SECRET_PATTERN: Regex =
        Regex::new(r#"let useFormkeyDecode=[\s\S]*?(window\.[\w]+="[^"]+")"#).unwrap();
    pub static ref STATIC_PATTERN: Regex = Regex::new(r#"static[^"]*\.js"#).unwrap();
    pub static ref BOT_NICKNAME: Regex = Regex::new(r#"nickname":"([^"]+)"#).unwrap();
}

/*
* +------+
* | BASE |
* +------+
*/

pub const BASE_URL: &str = "https://poe.com";
pub const DEFAULT_CATEGORY_NAME: &str = "defaultCategory";

pub fn default_headers() -> HeaderMap<HeaderValue> {
    let mut map = HeaderMap::new();
    map.insert(header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203"));
    map.insert(header::ACCEPT, HeaderValue::from_static("*/*"));
    map.insert(
        header::ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en,q=0.5"),
    );
    map.insert(
        "Sec-Ch-Ua",
        HeaderValue::from_static(
            "\"Microsoft Edge\";v=\"123\", \"Not:A-Brand\";v=\"8\", \"Chromium\";v=\"123\"",
        ),
    );
    map.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
    map.insert(
        "Sec-Ch-Ua-Platform",
        HeaderValue::from_static("\"Windows\""),
    );
    map.insert(
        header::UPGRADE_INSECURE_REQUESTS,
        HeaderValue::from_static("1"),
    );
    map.insert(header::ORIGIN, HeaderValue::from_static("https://poe.com"));
    map.insert(
        header::REFERER,
        HeaderValue::from_static("https://poe.com/"),
    );
    map
}

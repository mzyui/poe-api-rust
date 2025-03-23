use std::{sync::Arc, time::Duration};

use reqwest::{cookie::Jar, Client, Url};
use scraper::{Html, Selector};

use crate::{
    constants::{
        default_headers, BASE_URL, FORM_KEY_PATTERN, STATIC_PATTERN, WINDOW_SECRET_PATTERN,
    },
    models::Token,
};

#[derive(Default)]
pub struct PoeBundle {
    client: Client,
    window: String,
    src_scripts: Vec<String>,
    webpack_script: Option<String>,
    from_key: String,
}

impl PoeBundle {
    pub fn new(token: &Token) -> anyhow::Result<Self> {
        let headers = default_headers();
        let url = Url::parse(BASE_URL)?;
        let jar = Arc::new(Jar::default());

        jar.add_cookie_str(&format!("p-b={}", token.p_b), &url);
        jar.add_cookie_str(&format!("p-lat={}", token.p_lat), &url);

        let client = Client::builder()
            .default_headers(headers)
            .cookie_provider(jar)
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            ..Default::default()
        })
    }

    async fn initialize_web_data(&mut self) -> anyhow::Result<()> {
        log::debug!("Collecting javascript source code");

        // Reset data
        self.from_key.clear();
        self.window =
            String::from("const window={document:{hack:1},navigator:{userAgent:'safari <3'}};");

        let response = self.client.get(BASE_URL).send().await?;
        let document = response.text().await?;
        let html = Html::parse_document(&document);
        let selector = Selector::parse("script").unwrap();

        for script_element in html.select(&selector) {
            self.process_script_element(script_element).await?;
        }

        Ok(())
    }

    async fn process_script_element(
        &mut self,
        script_element: scraper::element_ref::ElementRef<'_>,
    ) -> anyhow::Result<()> {
        // Periksa apakah element script memiliki atribut src
        if let Some(src) = script_element.attr("src").map(String::from) {
            // Jika sudah pernah diproses, lewati
            if self.src_scripts.contains(&src) {
                return Ok(());
            }

            // Memproses src dengan logika khusus
            if src.contains("app") {
                self.init_app(&src).await?;
            } else if src.contains("buildManifest") {
                self.extend_src_scripts(&src).await?;
            } else if src.contains("webpack") {
                self.webpack_script = Some(src.clone());
                self.extend_src_scripts(&src).await?;
            } else {
                self.src_scripts.push(src);
            }
        } else {
            // Mengolah inline script
            let script_text = script_element.text().next().unwrap_or_default();
            if script_text.is_empty()
                || script_text.contains("document.")
                || !script_text.contains("function")
            {
                return Ok(());
            }
            if let Some(script_type) = script_element.attr("type") {
                if script_type == "application/json" {
                    return Ok(());
                }
            }
            self.window.push_str(script_text);
        }

        Ok(())
    }

    async fn init_app(&mut self, src: &str) -> anyhow::Result<()> {
        let script_content = self.load_src_script(src).await?;
        if let Some(window_secret_match) = WINDOW_SECRET_PATTERN.captures(&script_content) {
            if let Some(secret) = window_secret_match.get(1).map(|m| m.as_str()) {
                self.window.push_str(secret);
                self.window.push(';');
                return Ok(());
            }
        }
        anyhow::bail!("Failed to find window secret in js scripts")
    }

    async fn load_src_script(&self, src: &str) -> anyhow::Result<String> {
        let response = self.client.get(src).send().await?;
        let document = response.text().await?;
        Ok(document)
    }

    async fn extend_src_scripts(&mut self, manifest_src: &str) -> anyhow::Result<()> {
        let base_url = self.get_base_url(manifest_src)?;
        let manifest = self.load_src_script(manifest_src).await?;

        for cap in STATIC_PATTERN.captures_iter(&manifest) {
            if let Some(src_match) = cap.get(0) {
                let full_src = format!("{}/{}", base_url, src_match.as_str());
                self.src_scripts.push(full_src);
            }
        }
        Ok(())
    }

    fn get_base_url(&self, src: &str) -> anyhow::Result<String> {
        if let Some(base_url) = src.split("static/").next() {
            return Ok(base_url.to_string());
        }
        anyhow::bail!("Failed to find base url in manifest src")
    }

    pub async fn get_form_key(&mut self) -> anyhow::Result<String> {
        if self.window.is_empty() {
            self.initialize_web_data().await?;
        }
        if !self.from_key.is_empty() {
            return Ok(self.from_key.clone());
        }
        if let Some(cap) = FORM_KEY_PATTERN.captures(&self.window) {
            if let Some(func_name) = cap.get(1).map(|m| m.as_str()) {
                let script = format!("{}window.{}().slice(0, 32)", self.window, func_name);
                let context = quickjs_rs::Context::new()?;
                if let Some(value) = context.eval(&script)?.as_str().map(String::from) {
                    self.from_key = value.clone();
                    log::info!("Retrieved form key successfully: {}", self.from_key);
                    return Ok(value);
                }
            }
        }
        anyhow::bail!("Failed to parse form-key function in Poe document")
    }
}

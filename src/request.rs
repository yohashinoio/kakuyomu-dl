use anyhow::{anyhow, Result};

pub fn fetch_html(url: &str) -> Result<String> {
    let html = reqwest::blocking::get(url)?.text()?;

    if html.is_empty() {
        return Err(anyhow!("HTMLの取得に失敗しました: {}", url));
    }

    Ok(html)
}

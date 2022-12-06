use anyhow::Result;

pub fn fetch_html(url: &str) -> Result<String> {
    Ok(reqwest::blocking::get(url)?.text()?)
}

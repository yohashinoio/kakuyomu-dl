use anyhow::{anyhow, Result};

pub static KAKUYOMU_URL_HOST: &str = "kakuyomu.jp";

pub fn verify_url(url: &str) -> Result<()> {
    let res = url::Url::parse(url);

    let parsed = match res {
        Ok(psd) => psd,
        Err(_) => return Err(anyhow!("URLの解析に失敗しました")),
    };

    let host = match parsed.host_str() {
        None => return Err(anyhow!("URLのホスト名の取得に失敗しました")),
        Some(host) => host,
    };

    if !host.contains(KAKUYOMU_URL_HOST) {
        return Err(anyhow!("カクヨムのURLではありません"));
    }

    Ok(())
}

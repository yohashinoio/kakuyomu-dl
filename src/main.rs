mod dl;
mod novel;
mod request;
mod utils;

use anyhow::{anyhow, Result};
use novel::Novel;
use utils::verify_url;

fn main() -> Result<()> {
    let toc_url = match std::env::args().nth(1) {
        Some(x) => x,
        None => return Err(anyhow!("コマンドライン引数に目次のURLを指定してください")),
    };

    verify_url(&toc_url)?;

    dl::dl_novel(&Novel::parse(&toc_url)?)?;

    Ok(())
}

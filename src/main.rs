mod dl;
mod novel;
mod request;
mod utils;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use novel::Novel;
use utils::verify_url;

fn create_prog_style() -> ProgressStyle {
    ProgressStyle::with_template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>5}/{len:5} {msg}")
        .unwrap()
        .progress_chars("##>-")
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(anyhow!("コマンドライン引数に目次のURLを指定してください"));
    }

    let pbs = Arc::new(MultiProgress::new());

    let mut handles = Vec::new();

    for toc_url in args.into_iter().skip(1) {
        let pbs = pbs.clone();
        let pb = Arc::new(pbs.add(ProgressBar::new(0)));
        pb.set_style(create_prog_style());

        handles.push(std::thread::spawn(move || -> Result<()> {
            verify_url(&toc_url)?;

            dl::dl_novel(&Novel::parse(&toc_url, &pb)?)?;

            Ok(())
        }));
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}

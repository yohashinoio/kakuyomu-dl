mod dl;
mod novel;
mod request;
mod utils;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use clap::Parser;
use dl::DownloadOptions;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use novel::NovelInfo;
use utils::verify_url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, help = "出力されるファイル名の先頭にエピソード番号を付与します")]
    output_with_index: bool,

    #[arg(
        long,
        short,
        help = "どのエピソードからダウンロードを始めるか指定できます (1始まり)"
    )]
    begin: Option<i32>,

    #[arg(
        long,
        short,
        help = "どのエピソードまでダウンロードするか指定できます (1始まり)"
    )]
    end: Option<i32>,

    #[arg(value_name = "URL")]
    urls: Vec<String>,
}

fn create_prog_style() -> ProgressStyle {
    ProgressStyle::with_template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>5}/{len:5} {msg}")
        .unwrap()
        .progress_chars("##>-")
}

fn main() -> Result<()> {
    let args = Args::parse();

    let toc_urls = args.urls;

    let output_with_index = args.output_with_index;

    if toc_urls.is_empty() {
        return Err(anyhow!("コマンドライン引数に目次のURLを指定してください"));
    }

    let pbs = Arc::new(MultiProgress::new());

    let mut handles = Vec::new();

    for toc_url in toc_urls {
        let pbs = pbs.clone();
        let pb = Arc::new(pbs.add(ProgressBar::new(0)));
        pb.set_style(create_prog_style());

        handles.push(std::thread::spawn(move || -> Result<()> {
            verify_url(&toc_url)?;

            dl::dl_novel(
                &NovelInfo::fetch(&toc_url)?,
                &DownloadOptions {
                    output_with_index,
                    begin_episode: args.begin,
                    end_episode: args.end,
                },
                &pb,
            )?;

            Ok(())
        }));
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}

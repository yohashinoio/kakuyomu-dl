use std::{io::Write, sync::Arc};

use anyhow::{anyhow, Result};
use camino::Utf8PathBuf;

static KAKUYOMU_URL_HOST: &str = "kakuyomu.jp";

fn create_dir(path: &Utf8PathBuf) -> Result<()> {
    std::fs::create_dir_all(path)?;

    Ok(())
}

fn fetch_html(url: &str) -> Result<String> {
    Ok(reqwest::blocking::get(url)?.text()?)
}

fn verify_kakuyomu_url(url: &str) -> Result<()> {
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

fn get_episode_urls(url: &str) -> Result<Vec<String>> {
    let document = scraper::Html::parse_document(&fetch_html(url)?);

    let mut urls = Vec::new();

    for node in
        document.select(&scraper::Selector::parse(".widget-toc-episode-episodeTitle").unwrap())
    {
        urls.push(format!(
            "https://{}{}",
            KAKUYOMU_URL_HOST,
            node.value().attr("href").unwrap().to_string()
        ));
    }

    Ok(urls)
}

fn get_worktitle(toc_url: &str) -> Result<String> {
    let document = scraper::Html::parse_document(&fetch_html(toc_url)?);

    for node in document.select(&scraper::Selector::parse("#workTitle").unwrap()) {
        return Ok(node.text().collect::<Vec<&str>>()[0].to_string());
    }

    Err(anyhow!("作品タイトルの取得に失敗しました"))
}

fn get_episode_title(episode_html: &scraper::Html) -> Result<String> {
    for node in episode_html.select(&scraper::Selector::parse(".widget-episodeTitle").unwrap()) {
        return Ok(node.text().collect::<Vec<&str>>()[0].to_string());
    }

    Err(anyhow!("エピソードタイトルの取得に失敗しました"))
}

fn get_episode_main_text(episode_html: &scraper::Html) -> String {
    let mut main_text = String::new();

    for node in episode_html.select(&scraper::Selector::parse(".js-episode-body").unwrap()) {
        for text in node.text().collect::<Vec<&str>>() {
            main_text += text;
        }
    }

    main_text
}

fn download_episode(episode_url: &str, episode_idx: usize, dl_path: &Utf8PathBuf) -> Result<()> {
    let episode_doc = scraper::Html::parse_document(&fetch_html(episode_url)?);

    let episode_title = get_episode_title(&episode_doc)?;

    let filename = format!("{} {}.txt", episode_idx, episode_title);

    let mut file = std::fs::File::create(dl_path.join(filename))?;

    file.write_all(get_episode_main_text(&episode_doc).as_bytes())?;

    Ok(())
}

fn create_pb_style() -> indicatif::ProgressStyle {
    indicatif::ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:50.cyan/blue} {pos:>5}/{len:5} {msg}",
    )
    .unwrap()
    .progress_chars("##>-")
}

fn download_episodes(episode_urls: Vec<String>, dl_path: Utf8PathBuf) -> Result<()> {
    let pb = Arc::new(indicatif::ProgressBar::new(episode_urls.len() as u64));
    let dl_path = Arc::new(dl_path);

    let mut handles = Vec::new();

    pb.set_style(create_pb_style());
    pb.reset_eta();

    for (idx, url) in episode_urls.into_iter().enumerate() {
        let pb = pb.clone();
        let dl_path = dl_path.clone();

        handles.push(std::thread::spawn(move || -> Result<()> {
            download_episode(&url, idx + 1, &dl_path)?;
            pb.inc(1);
            Ok(())
        }));
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    pb.finish_with_message("Done!");

    Ok(())
}

fn download_kakuyomu_novel(toc_url: &str) -> Result<()> {
    let worktitle = get_worktitle(&toc_url)?;

    let dl_path = Utf8PathBuf::from("output").join(worktitle);

    create_dir(&dl_path)?;

    download_episodes(get_episode_urls(&toc_url)?, dl_path)?;

    Ok(())
}

fn main() -> Result<()> {
    // URL to table of contents
    let toc_url = match std::env::args().nth(1) {
        Some(x) => x,
        None => return Err(anyhow!("コマンドライン引数に目次のURLを指定してください")),
    };

    verify_kakuyomu_url(&toc_url)?;

    download_kakuyomu_novel(&toc_url)?;

    Ok(())
}

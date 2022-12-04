use core::panic;
use std::{io::Write, sync::Arc};

static KAKUYOMU_HOST_URL: &str = "kakuyomu.jp";

fn create_dir(path: &str) {
    match std::fs::create_dir_all(path) {
        Err(why) => panic!("{}", why),
        Ok(_) => {}
    };
}

fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::blocking::get(url)?.text()?)
}

fn verify_kakuyomu_url(url: &str) {
    let res = url::Url::parse(url);

    let parsed = match res {
        Ok(psd) => psd,
        Err(_) => panic!("Failed to parse the URL"),
    };

    if !parsed.host_str().unwrap().contains(KAKUYOMU_HOST_URL) {
        panic!("Not a URL for Kakuyomu");
    };
}

fn get_episode_urls(url: &str) -> Vec<String> {
    let html = fetch_html(url);

    let html = match html {
        Ok(text) => text,
        Err(error) => panic!("{}", error),
    };

    let document = scraper::Html::parse_document(&html);

    let mut urls: Vec<String> = Vec::new();

    for node in
        document.select(&scraper::Selector::parse(".widget-toc-episode-episodeTitle").unwrap())
    {
        urls.push(format!(
            "https://{}{}",
            KAKUYOMU_HOST_URL,
            node.value().attr("href").unwrap().to_string()
        ));
    }

    urls
}

fn get_worktitle(toc_url: &str) -> Option<String> {
    let html = fetch_html(toc_url);

    let html = match html {
        Ok(text) => text,
        Err(_) => return None,
    };

    let document = scraper::Html::parse_document(&html);

    for node in document.select(&scraper::Selector::parse("#workTitle").unwrap()) {
        return Some(node.text().collect::<Vec<&str>>()[0].to_string());
    }

    None
}

fn get_episode_title(episode_html: &scraper::Html) -> Option<String> {
    for node in episode_html.select(&scraper::Selector::parse(".widget-episodeTitle").unwrap()) {
        return Some(node.text().collect::<Vec<&str>>()[0].to_string());
    }

    None
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

fn download_episode(episode_url: &str, episode_idx: usize, output_path: &str) {
    let episode_doc = scraper::Html::parse_document(&match fetch_html(episode_url) {
        Ok(text) => text,
        Err(_) => panic!(
            "Failed to parse episode document: episode '{}'",
            episode_idx
        ),
    });

    let episode_title = match get_episode_title(&episode_doc) {
        Some(title) => title,
        None => panic!("Failed to fetch episode title: episode '{}'", episode_idx),
    };

    let filename = format!("{} {}", episode_idx, episode_title);

    let mut file = match std::fs::File::create(format!("{}/{}", output_path, filename)) {
        Ok(file) => file,
        Err(_) => panic!("Failed to create episode file: episode '{}'", episode_idx),
    };

    match file.write_all(get_episode_main_text(&episode_doc).as_bytes()) {
        Ok(_) => (),
        Err(_) => panic!(
            "Failed to write episode main text: episode '{}'",
            episode_idx
        ),
    };
}

fn download_episodes(episode_urls: Vec<String>, output_path: String) {
    let output_path = Arc::new(output_path);
    let pb = Arc::new(indicatif::ProgressBar::new(episode_urls.len() as u64));

    let style = indicatif::ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:50.cyan/blue} {pos:>5}/{len:5} {msg}",
    )
    .unwrap()
    .progress_chars("##>-");

    pb.set_style(style);
    pb.reset_eta();

    let mut handles = vec![];

    for (idx, url) in episode_urls.iter().cloned().enumerate() {
        let output_path = Arc::clone(&output_path);
        let pb = Arc::clone(&pb);

        handles.push(std::thread::spawn(move || {
            download_episode(&url, idx + 1, output_path.as_str());
            pb.inc(1);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    pb.finish_with_message("Done!");
}

fn download_novel(toc_url: &str) {
    let worktitle = match get_worktitle(&toc_url) {
        Some(title) => title,
        None => panic!("Failed to fetch work title"),
    };

    let output_path = format!("output/{}", worktitle);

    create_dir(&output_path);

    download_episodes(get_episode_urls(&toc_url), output_path);
}

fn main() {
    let toc_url = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => panic!("Specify a URL as the first command line argument"),
    };

    verify_kakuyomu_url(&toc_url);

    download_novel(&toc_url);
}

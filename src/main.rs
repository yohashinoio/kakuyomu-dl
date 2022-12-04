use core::panic;
use std::io::Write;

static KAKUYOMU_HOST_URL: &str = "kakuyomu.jp";

fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::blocking::get(url)?.text()?)
}

fn is_valid_kakuyomu_url(url: &str) -> Result<(), &str> {
    let res = url::Url::parse(url);

    // psd stands for parsed
    let psd = match res {
        Ok(psd) => psd,
        Err(_) => return Err("URL parsing failed"),
    };

    if !psd.host_str().unwrap().contains(KAKUYOMU_HOST_URL) {
        return Err("Not a URL for Kakuyomu");
    };

    Ok(())
}

fn get_episode_urls(url: &str) -> Vec<String> {
    let html = get_html(url);

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

fn get_worktitle(episode_list_url: &str) -> Option<String> {
    let html = get_html(episode_list_url);

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

fn create_dir(path: &str) {
    match std::fs::create_dir(path) {
        Err(why) => panic!("{}", why),
        Ok(_) => {}
    };
}

fn main() {
    let url = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => panic!("Specify a URL as the first command line argument"),
    };

    match is_valid_kakuyomu_url(&url) {
        Err(_) => panic!("Please specify a valid URL"),
        Ok(_) => {}
    };

    let worktitle = match get_worktitle(&url) {
        Some(t) => t,
        None => panic!("Failed to fetch work title"),
    };

    create_dir(&worktitle);

    for (idx, url) in get_episode_urls(&url).iter().enumerate() {
        let episode_idx = idx + 1;

        let episode_doc = scraper::Html::parse_document(&match get_html(&url) {
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

        let mut file = match std::fs::File::create(format!("{}/{}", &worktitle, &episode_title)) {
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
}

use anyhow::{anyhow, Result};
use scraper::Html;

use crate::{request::fetch_html, utils::KAKUYOMU_URL_HOST};

#[derive(Debug)]
pub struct NovelInfo {
    worktitle: String,
    episode_urls: Vec<String>,
}

impl NovelInfo {
    pub fn get_worktitle(&self) -> &str {
        &self.worktitle
    }

    pub fn get_episode_urls(&self) -> &Vec<String> {
        &self.episode_urls
    }

    pub fn fetch(toc_url: &str) -> Result<Self> {
        let toc_doc = scraper::Html::parse_document(&fetch_html(toc_url)?);

        let worktitle = Self::parse_worktitle(&toc_doc)?;

        Ok(Self {
            episode_urls: Self::parse_episode_urls(&toc_doc)?,
            worktitle,
        })
    }

    fn parse_episode_urls(toc_doc: &Html) -> Result<Vec<String>> {
        let mut urls = Vec::new();

        for node in
            toc_doc.select(&scraper::Selector::parse(".widget-toc-episode-episodeTitle").unwrap())
        {
            urls.push(format!(
                "https://{}{}",
                KAKUYOMU_URL_HOST,
                node.value().attr("href").unwrap()
            ));
        }

        Ok(urls)
    }

    fn parse_worktitle(toc_doc: &Html) -> Result<String> {
        if let Some(node) = toc_doc
            .select(&scraper::Selector::parse("#workTitle").unwrap())
            .next()
        {
            return Ok(node.text().collect::<Vec<&str>>()[0].to_string());
        }

        Err(anyhow!("作品タイトルの取得に失敗しました"))
    }
}

#[derive(Debug)]
pub struct Episode {
    idx: usize, // Beginning 1
    title: String,
    main_text: String,
}

impl Episode {
    pub fn get_index(&self) -> usize {
        self.idx
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_main_text(&self) -> &str {
        &self.main_text
    }

    pub fn fetch(url: &str, idx: usize) -> Result<Self> {
        let doc = scraper::Html::parse_document(&fetch_html(url)?);

        Ok(Self {
            idx,
            title: Self::parse_title(&doc)?,
            main_text: Self::parse_main_text(&doc),
        })
    }

    fn parse_title(doc: &Html) -> Result<String> {
        if let Some(node) = doc
            .select(&scraper::Selector::parse(".widget-episodeTitle").unwrap())
            .next()
        {
            return Ok(node.text().collect::<Vec<&str>>()[0].to_string());
        }

        Err(anyhow!("エピソードタイトルの取得に失敗しました"))
    }

    fn parse_main_text(doc: &Html) -> String {
        let mut main_text = String::new();

        for node in doc.select(&scraper::Selector::parse(".js-episode-body").unwrap()) {
            for text in node.text().collect::<Vec<&str>>() {
                main_text += text;
            }
        }

        main_text
    }
}

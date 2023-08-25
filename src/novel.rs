use std::sync::Arc;

use anyhow::{anyhow, Result};
use indicatif::ProgressBar;
use scraper::Html;

use crate::{request::fetch_html, utils::KAKUYOMU_URL_HOST};

#[derive(Debug)]
pub struct Novel {
    worktitle: String,
    episodes: Vec<Episode>,
}

impl Novel {
    pub fn get_worktitle(&self) -> &str {
        &self.worktitle
    }

    pub fn get_episodes(&self) -> &Vec<Episode> {
        &self.episodes
    }

    pub fn parse(toc_url: &str, pb: &Arc<ProgressBar>) -> Result<Self> {
        let toc_doc = scraper::Html::parse_document(&fetch_html(toc_url)?);

        let worktitle = Self::parse_worktitle(&toc_doc)?;

        Ok(Self {
            episodes: Self::parse_episodes(&toc_doc, &worktitle, pb)?,
            worktitle: worktitle,
        })
    }

    // The length of the ProgressBar is set within this function
    fn parse_episodes(
        toc_doc: &Html,
        worktitle: &str,
        pb: &Arc<ProgressBar>,
    ) -> Result<Vec<Episode>> {
        let episode_urls = Self::parse_episode_urls(&toc_doc)?;

        let mut episodes = Vec::new();

        pb.set_length(episode_urls.len() as u64);
        pb.set_message(worktitle.to_string());

        for (idx, url) in episode_urls.into_iter().enumerate() {
            let pb = pb.clone();

            let episode_idx = idx + 1;

            let ep = Episode::parse(&url, episode_idx)?;
            episodes.push(ep);
            pb.inc(1);
        }

        pb.finish_with_message("Done!");

        Ok(episodes)
    }

    fn parse_episode_urls(toc_doc: &Html) -> Result<Vec<String>> {
        let mut urls = Vec::new();

        for node in
            toc_doc.select(&scraper::Selector::parse(".widget-toc-episode-episodeTitle").unwrap())
        {
            urls.push(format!(
                "https://{}{}",
                KAKUYOMU_URL_HOST,
                node.value().attr("href").unwrap().to_string()
            ));
        }

        Ok(urls)
    }

    fn parse_worktitle(toc_doc: &Html) -> Result<String> {
        for node in toc_doc.select(&scraper::Selector::parse("#workTitle").unwrap()) {
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

    pub fn parse(url: &str, idx: usize) -> Result<Self> {
        let doc = scraper::Html::parse_document(&fetch_html(url)?);

        Ok(Self {
            idx,
            title: Self::parse_title(&doc)?,
            main_text: Self::parse_main_text(&doc),
        })
    }

    fn parse_title(doc: &Html) -> Result<String> {
        for node in doc.select(&scraper::Selector::parse(".widget-episodeTitle").unwrap()) {
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

use std::{io::Write, sync::Arc};

use anyhow::{anyhow, Result};
use camino::Utf8PathBuf;
use indicatif::ProgressBar;

use crate::novel::{Episode, NovelInfo};

fn create_dir(path: &Utf8PathBuf) -> Result<()> {
    std::fs::create_dir_all(path)?;

    Ok(())
}

pub struct DownloadOptions {
    pub output_with_index: bool,
    pub begin_episode: Option<i32>,
    pub end_episode: Option<i32>,
}

// ep stands for episode
fn format_ep_filename(idx: usize, title: &str, options: &DownloadOptions) -> String {
    if options.output_with_index {
        format!("{} {}.txt", idx, title)
    } else {
        format!("{}.txt", title)
    }
}

fn dl_episode(ep: &Episode, dl_path: &Utf8PathBuf, options: &DownloadOptions) -> Result<()> {
    std::fs::File::create(dl_path.join(format_ep_filename(
        ep.get_index(),
        ep.get_title(),
        options,
    )))?
    .write_all(ep.get_main_text().as_bytes())?;

    Ok(())
}

pub fn dl_novel(novel: &NovelInfo, options: &DownloadOptions, pb: &Arc<ProgressBar>) -> Result<()> {
    let dl_path = Utf8PathBuf::from("output").join(novel.get_worktitle());

    create_dir(&dl_path)?;

    let episode_urls = novel.get_episode_urls();

    let begin_episode = options.begin_episode.unwrap_or(1);
    let end_episode = options.end_episode.unwrap_or(episode_urls.len() as i32);

    let pb_len = end_episode - begin_episode + 1;
    if pb_len <= 0 {
        return Err(anyhow!("Make sure that 'begin' may be larger than 'end'"));
    }

    pb.set_length(pb_len as u64);
    pb.set_message(novel.get_worktitle().to_string());

    let mut idx = begin_episode;

    loop {
        if end_episode < idx {
            break;
        }

        let episode = Episode::fetch(&episode_urls[(idx - 1) as usize], idx as usize)?;

        dl_episode(&episode, &dl_path, options)?;

        idx += 1;
        pb.inc(1);
    }

    pb.finish_with_message("Done!");

    Ok(())
}

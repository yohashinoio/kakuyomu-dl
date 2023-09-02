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
    pub start: Option<i32>,
    pub finish: Option<i32>,
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

    pb.set_message(novel.get_worktitle().to_string());

    let start_idx = options.start.unwrap_or(1);
    let finish_idx = options.finish.unwrap_or(episode_urls.len() as i32);

    let pb_len = finish_idx - start_idx + 1;
    if pb_len <= 0 {
        return Err(anyhow!("'start'が'finish'よりも大きい可能性があります"));
    }
    pb.set_length(pb_len as u64);

    let mut idx = start_idx;
    loop {
        if finish_idx < idx {
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

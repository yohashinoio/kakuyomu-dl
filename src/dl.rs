use std::io::Write;

use anyhow::Result;
use camino::Utf8PathBuf;

use crate::novel::{Episode, Novel};

fn create_dir(path: &Utf8PathBuf) -> Result<()> {
    std::fs::create_dir_all(path)?;

    Ok(())
}

// ep stands for episode
fn format_ep_filename(idx: usize, ep_title: &str) -> String {
    format!("{} {}.txt", idx, ep_title)
}

fn dl_episode(ep: &Episode, dl_path: &Utf8PathBuf) -> Result<()> {
    std::fs::File::create(dl_path.join(format_ep_filename(ep.get_index(), ep.get_title())))?
        .write_all(ep.get_main_text().as_bytes())?;

    Ok(())
}

pub fn dl_novel(novel: &Novel) -> Result<()> {
    let dl_path = Utf8PathBuf::from("output").join(novel.get_worktitle());

    create_dir(&dl_path)?;

    for ep in novel.get_episodes() {
        dl_episode(ep, &dl_path)?;
    }

    Ok(())
}

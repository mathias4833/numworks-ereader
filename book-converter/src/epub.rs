use anyhow::{Context, Result};
use epub::doc::EpubDoc;
use std::path::Path;

pub struct EpubBook {
    pub title: String,
    pub author: String,
    pub chapters: Vec<String>,
    pub cover: Option<Vec<u8>>,
}

impl EpubBook {
    pub fn open(path: &Path) -> Result<Self> {
        let mut epub = EpubDoc::new(path).context("failed to open EPUB")?;
        let title = epub.get_title().context("EPUB has no title")?;
        let author = epub
            .mdata("creator")
            .map(|item| item.value.clone())
            .unwrap_or_default();
        let cover = epub.get_cover().map(|(bytes, _)| bytes);
        let nav_id = epub.get_nav_id();

        let mut chapters = Vec::new();
        for index in 0..epub.spine.len() {
            let item = &epub.spine[index];
            if !item.linear || nav_id.as_deref() == Some(item.idref.as_str()) {
                continue;
            }

            epub.set_current_chapter(index);

            if epub.get_current_mime().as_deref() != Some("application/xhtml+xml") {
                continue;
            }

            let (html, _) = epub
                .get_current_str()
                .context("failed to read EPUB chapter")?;
            let text = html2text::from_read(html.as_bytes(), 10_000)
                .context("failed to extract EPUB text")?;
            if !text.trim().is_empty() {
                chapters.push(text);
            }
        }

        Ok(Self {
            title,
            author,
            chapters,
            cover,
        })
    }
}

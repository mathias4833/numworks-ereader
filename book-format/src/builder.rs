use crate::{MAGIC, VERSION};
use std::string::String;
use std::vec::Vec;

#[derive(Debug)]
pub enum EncodeError {
    TooLarge,
}

pub struct BookBuilder {
    title: String,
    pages: Vec<String>,
}

impl BookBuilder {
    pub fn new(title: String, pages: Vec<String>) -> Self {
        Self { title, pages }
    }

    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        let page_count = u32::try_from(self.pages.len()).map_err(|_| EncodeError::TooLarge)?;
        let title_len = u32::try_from(self.title.len()).map_err(|_| EncodeError::TooLarge)?;

        let mut data = Vec::new();

        data.extend_from_slice(MAGIC);
        data.extend_from_slice(&VERSION.to_le_bytes());
        data.extend_from_slice(&page_count.to_le_bytes());
        data.extend_from_slice(&title_len.to_le_bytes());

        data.extend_from_slice(self.title.as_bytes());

        let mut offset = 0u32;
        for page in &self.pages {
            data.extend_from_slice(&offset.to_le_bytes());
            let page_len = u32::try_from(page.len()).map_err(|_| EncodeError::TooLarge)?;
            offset = offset.checked_add(page_len).ok_or(EncodeError::TooLarge)?;
        }
        data.extend_from_slice(&offset.to_le_bytes());

        for page in &self.pages {
            data.extend_from_slice(page.as_bytes());
        }

        Ok(data)
    }
}

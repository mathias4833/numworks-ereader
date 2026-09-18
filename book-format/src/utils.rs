#[cfg(feature = "std")]
use crate::EncodeError;
use core::ops::Range;
#[cfg(feature = "std")]
use std::vec::Vec;

pub(crate) fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    let bytes = data.get(offset..offset + 4)?;

    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub(crate) struct OffsetTable<'a> {
    data: &'a [u8],
}

impl<'a> OffsetTable<'a> {
    pub fn read(reader: &mut Reader<'a>, count: usize) -> Option<Self> {
        let len = count.checked_add(1)?.checked_mul(size_of::<u32>())?;

        Some(Self {
            data: reader.take(len)?,
        })
    }

    pub const fn len(&self) -> usize {
        self.data.len() / size_of::<u32>() - 1
    }

    fn offset(&self, index: usize) -> Option<usize> {
        read_u32(self.data, index.checked_mul(size_of::<u32>())?)?
            .try_into()
            .ok()
    }

    pub fn range(&self, index: usize) -> Option<Range<usize>> {
        if index >= self.len() {
            return None;
        }

        let start = self.offset(index)?;
        let end = self.offset(index + 1)?;

        Some(start..end)
    }
}

pub(crate) struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn take(&mut self, len: usize) -> Option<&'a [u8]> {
        let end = self.pos.checked_add(len)?;
        let slice = self.data.get(self.pos..end)?;
        self.pos = end;
        Some(slice)
    }

    pub fn u16(&mut self) -> Option<u16> {
        let bytes = self.take(2)?;
        Some(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn u32(&mut self) -> Option<u32> {
        let bytes = self.take(4)?;
        Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn remaining(&self) -> &'a [u8] {
        &self.data[self.pos..]
    }
}

#[cfg(feature = "std")]
pub(crate) struct Writer {
    data: Vec<u8>,
}

#[cfg(feature = "std")]
impl Writer {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn finish(self) -> Vec<u8> {
        self.data
    }

    pub fn bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn u16(&mut self, value: u16) {
        self.bytes(&value.to_le_bytes());
    }

    pub fn u32(&mut self, value: usize) -> Result<(), EncodeError> {
        let value = u32::try_from(value).map_err(|_| EncodeError::TooLarge)?;
        self.bytes(&value.to_le_bytes());

        Ok(())
    }

    pub fn indexed<T>(
        &mut self,
        items: &[T],
        mut write: impl FnMut(&mut Self, &T) -> Result<(), EncodeError>,
    ) -> Result<(), EncodeError> {
        let offset_count = items.len().checked_add(1).ok_or(EncodeError::TooLarge)?;
        let table_len = offset_count
            .checked_mul(size_of::<u32>())
            .ok_or(EncodeError::TooLarge)?;
        let table_start = self.data.len();

        let payload_start = table_start
            .checked_add(table_len)
            .ok_or(EncodeError::TooLarge)?;

        self.data.resize(payload_start, 0);

        for (index, item) in items.iter().enumerate() {
            self.write_offset(table_start, index, payload_start)?;
            write(self, item)?;
        }

        self.write_offset(table_start, items.len(), payload_start)?;

        Ok(())
    }

    fn write_offset(
        &mut self,
        table_start: usize,
        index: usize,
        payload_start: usize,
    ) -> Result<(), EncodeError> {
        let offset = self
            .data
            .len()
            .checked_sub(payload_start)
            .ok_or(EncodeError::TooLarge)?;
        let offset = u32::try_from(offset).map_err(|_| EncodeError::TooLarge)?;

        let position = index
            .checked_mul(size_of::<u32>())
            .and_then(|offset| table_start.checked_add(offset))
            .ok_or(EncodeError::TooLarge)?;

        let end = position
            .checked_add(size_of::<u32>())
            .ok_or(EncodeError::TooLarge)?;

        self.data[position..end].copy_from_slice(&offset.to_le_bytes());

        Ok(())
    }
}

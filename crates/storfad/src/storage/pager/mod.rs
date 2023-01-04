use bytes::{Buf, BytesMut};
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, AsyncWrite, AsyncWriteExt};

#[derive(Error, Debug)]
pub enum Error {
    #[error("i/o error: {0}")]
    IoError(#[from] std::io::Error),
}

pub trait File: AsyncRead + AsyncWrite + AsyncSeek {}

pub struct PagedFile<F: File + Unpin> {
    file: F,
    page_size: usize,
}

impl<F: File + Unpin> PagedFile<F> {
    pub fn new(file: F, page_size: usize) -> Self {
        PagedFile {
            file,
            page_size: page_size,
        }
    }

    /// Reads the specified page from the file.
    pub async fn read(&mut self, page: usize) -> Result<BytesMut, Error> {
        let offset = page as u64 * self.page_size as u64;
        self.file
            .seek(std::io::SeekFrom::Start(offset as u64))
            .await?;

        let mut bytes = BytesMut::with_capacity(self.page_size as usize);
        bytes.resize(self.page_size as usize, 0);

        self.file.read_exact(&mut bytes[..]).await?;
        Ok(bytes)
    }

    pub async fn write(&mut self, page: usize, mut content: impl Buf) -> Result<(), Error> {
        if content.remaining() > self.page_size as usize {
            return Err(Error::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("content is larger than the page size: {}", self.page_size),
            )));
        }
        let offset = page as u64 * self.page_size as u64;
        self.file
            .seek(std::io::SeekFrom::Start(offset as u64))
            .await?;

        self.file.write_all_buf(&mut content).await?;
        self.file.flush().await?;
        Ok(())
    }
}

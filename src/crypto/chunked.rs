use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use anyhow::Context as _;

pub(crate) struct Chunked<const LEN: usize, Inner>
where
    Inner: Read,
{
    pub(crate) inner: BufReader<Inner>,
}

impl<const LEN: usize, Inner> Chunked<LEN, Inner>
where
    Inner: Read,
{
    pub(crate) fn new(inner: Inner) -> Self {
        Self {
            inner: BufReader::with_capacity(LEN, inner),
        }
    }
}

impl<const LEN: usize, Inner> Iterator for Chunked<LEN, Inner>
where
    Inner: Read,
{
    type Item = anyhow::Result<heapless::Vec<u8, LEN>>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.inner.fill_buf().context("reading chunk");

        match bytes {
            Ok(&[]) => None,
            Ok(bytes) => {
                let output = heapless::Vec::<u8, LEN>::from_slice(bytes).unwrap();

                self.inner.consume(output.len());

                Some(Ok(output))
            }
            Err(err) => Some(Err(err)),
        }
    }
}

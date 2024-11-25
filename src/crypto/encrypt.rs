use std::io::Read;
use std::ops::Sub;

use aes_gcm::aead::stream::NewStream;
use aes_gcm::aead::stream::StreamBE32;
use aes_gcm::aead::stream::StreamPrimitive;
use aes_gcm::aes::cipher::Unsigned;
use aes_gcm::AeadCore;
use aes_gcm::Aes256Gcm;
use aes_gcm::Key;
use aes_gcm::KeyInit as _;
use aes_gcm::KeySizeUser;
use aes_gcm::Nonce;
use anyhow::Context as _;
use typenum::U5;

use super::chunked::Chunked;
use crate::utils::try_from_fn::try_from_fn;

const KEY_SIZE: usize = <Aes256Gcm as KeySizeUser>::KeySize::USIZE;
const NONCE_SIZE: usize = <<Aes256Gcm as AeadCore>::NonceSize as Sub<U5>>::Output::USIZE;
pub const TAG_SIZE: usize = <Aes256Gcm as AeadCore>::TagSize::USIZE;

#[cfg(debug_assertions)]
const CHUNK_LEN: usize = 1;

#[cfg(not(debug_assertions))]
const CHUNK_LEN: usize = 1024; // 1 KiB

pub(crate) fn encrypt<'plaintext>(
    key: &[u8; KEY_SIZE],
    nonce: &[u8; NONCE_SIZE],
    additional_data: &'plaintext [u8],
    plaintext: impl Read + 'plaintext,
) -> impl Iterator<Item = anyhow::Result<heapless::Vec<u8, { CHUNK_LEN + TAG_SIZE }>>> + 'plaintext
{
    let key = Key::<Aes256Gcm>::from_slice(key);
    let nonce = Nonce::from_slice(nonce);
    let aead = Aes256Gcm::new(key);
    let stream_aead = StreamBE32::from_aead(aead, nonce);

    let mut plaintext_chunks = Chunked::<CHUNK_LEN, _>::new(plaintext)
        .enumerate()
        .peekable();

    try_from_fn(move || {
        if let Some((position, plaintext_chunk)) = plaintext_chunks.next() {
            let plaintext_chunk = plaintext_chunk?;

            let is_last_block = plaintext_chunks.peek().is_none();

            let mut buffer =
                heapless::Vec::<_, { CHUNK_LEN + TAG_SIZE }>::from_slice(&plaintext_chunk).unwrap();

            stream_aead
                .encrypt_in_place(position as u32, is_last_block, additional_data, &mut buffer)
                .map_err(|err| anyhow::anyhow!(err))
                .context("encrypting chunk")?;

            Ok(Some(buffer))
        } else {
            Ok(None)
        }
    })
}

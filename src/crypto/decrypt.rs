use std::io::Read;
use std::ops::Sub;

use aes_gcm::aead::stream::NewStream as _;
use aes_gcm::aead::stream::StreamBE32;
use aes_gcm::aead::stream::StreamPrimitive as _;
use aes_gcm::aes::cipher::Unsigned;
use aes_gcm::AeadCore;
use aes_gcm::Aes256Gcm;
use aes_gcm::Key;
use aes_gcm::KeyInit as _;
use aes_gcm::KeySizeUser;
use aes_gcm::Nonce;
use anyhow::Context as _;
use typenum::U5;

use crate::crypto::chunked::Chunked;
use crate::utils::try_from_fn::try_from_fn;

const KEY_SIZE: usize = <Aes256Gcm as KeySizeUser>::KeySize::USIZE;
const NONCE_SIZE: usize = <<Aes256Gcm as AeadCore>::NonceSize as Sub<U5>>::Output::USIZE;
const TAG_SIZE: usize = <Aes256Gcm as AeadCore>::TagSize::USIZE;

#[cfg(debug_assertions)]
const CHUNK_LEN: usize = 1;

#[cfg(not(debug_assertions))]
const CHUNK_LEN: usize = 1024; // 1 KiB

pub(crate) fn decrypt<'ciphertext>(
    key: &[u8; KEY_SIZE],
    nonce: &[u8; NONCE_SIZE],
    additional_data: &'ciphertext [u8],
    ciphertext: impl Read + 'ciphertext,
) -> impl Iterator<Item = anyhow::Result<heapless::Vec<u8, CHUNK_LEN>>> + 'ciphertext {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let nonce = Nonce::from_slice(nonce);
    let aead = Aes256Gcm::new(key);
    let stream_aead = StreamBE32::from_aead(aead, nonce);

    let mut ciphertext_chunks = Chunked::<{ CHUNK_LEN + TAG_SIZE }, _>::new(ciphertext)
        .enumerate()
        .peekable();

    try_from_fn(move || {
        if let Some((position, ciphertext_chunk)) = ciphertext_chunks.next() {
            let ciphertext_chunk = ciphertext_chunk?;

            let is_last_block = ciphertext_chunks.peek().is_none();

            let mut buffer =
                heapless::Vec::<_, { CHUNK_LEN + TAG_SIZE }>::from_slice(&ciphertext_chunk)
                    .unwrap();

            stream_aead
                .decrypt_in_place(position as u32, is_last_block, additional_data, &mut buffer)
                .map_err(|err| anyhow::anyhow!(err))
                .context("decrypting chunk")?;

            Ok(Some(heapless::Vec::from_slice(&buffer).unwrap()))
        } else {
            Ok(None)
        }
    })
}

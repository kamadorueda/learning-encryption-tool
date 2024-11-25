use anyhow::Context as _;

pub(crate) fn random_bytes<const LEN: usize>() -> anyhow::Result<[u8; LEN]> {
    // generate cryptographically secure random bytes
    let mut bytes = [0u8; LEN];

    getrandom::getrandom(&mut bytes)
        .map_err(|err| anyhow::anyhow!(err))
        .context("generating random bytes")?;

    Ok(bytes)
}
